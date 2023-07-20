use cursive::{
    direction::Orientation,
    view::{Nameable, Resizable},
    views::{Dialog, EditView, LinearLayout, NamedView, PaddedView, RadioGroup, ResizedView},
    Cursive, With,
};
use serde::{Deserialize, Serialize};

use crate::{
    database::model::ColType,
    server::model::User,
    tui::{
        components::{self, selector::update_select_component_with_ids},
        model::{Conn, Sidebar},
        utils::{self, get_current_model, get_current_mut_model},
    },
};

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct TotalUsers {
    pub id: i64,
}

pub fn auth_dashboard(s: &mut Cursive) -> NamedView<ResizedView<Dialog>> {
    let mut layout = LinearLayout::new(Orientation::Vertical);

    let users = get_all_users(s, "", 0);

    let users_email = users
        .clone()
        .into_iter()
        .map(|u| u.email)
        .collect::<Vec<String>>();

    let on_select = move |s: &mut Cursive, idx: &usize| {
        edit_role(s, *idx);
    };

    let on_refresh = |s: &mut Cursive| {
        let text = utils::get_data_from_refname::<EditView>(s, "searchUser")
            .get_content()
            .to_string();

        let model = get_current_mut_model(s);
        model.offset = 0;

        let users = &get_all_users(s, &text, 0);
        let users_email = users
            .clone()
            .into_iter()
            .map(|u| u.email)
            .collect::<Vec<String>>();

        let users_ids: Vec<usize> = users
            .clone()
            .into_iter()
            .map(|u| u.id as usize)
            .collect::<Vec<usize>>();

        update_select_component_with_ids(s, "user_list", users_email, users_ids);
    };

    let on_previous = |s: &mut Cursive| {
        {
            let model = get_current_mut_model(s);
            if model.offset > 25 {
                model.offset = model.offset - 25;
            } else {
                model.offset = 0;
            }
        }

        let text = utils::get_data_from_refname::<EditView>(s, "searchUser")
            .get_content()
            .to_string();

        let model = get_current_model(s);
        let users = &get_all_users(s, &text, model.offset);
        let users_email = users
            .clone()
            .into_iter()
            .map(|u| u.email)
            .collect::<Vec<String>>();

        let users_ids: Vec<usize> = users
            .clone()
            .into_iter()
            .map(|u| u.id as usize)
            .collect::<Vec<usize>>();

        update_select_component_with_ids(s, "user_list", users_email, users_ids);
    };

    let on_next = |s: &mut Cursive| {
        let text = utils::get_data_from_refname::<EditView>(s, "searchUser")
            .get_content()
            .to_string();

        {
            let total = get_total_users_count(s, &text);

            let model = get_current_mut_model(s);
            if model.offset + 25 < total {
                model.offset = model.offset + 25;
            }
        }

        let model = get_current_model(s);
        let users = &get_all_users(s, &text, model.offset);
        let users_email = users
            .clone()
            .into_iter()
            .map(|u| u.email)
            .collect::<Vec<String>>();

        let users_ids: Vec<usize> = users
            .clone()
            .into_iter()
            .map(|u| u.id as usize)
            .collect::<Vec<usize>>();

        update_select_component_with_ids(s, "user_list", users_email, users_ids);
    };

    let users_ids: Vec<usize> = users
        .into_iter()
        .map(|u| u.id as usize)
        .collect::<Vec<usize>>();

    layout.add_child(PaddedView::lrtb(
        1,
        1,
        0,
        1,
        EditView::new()
            .on_submit(|s, text| {
                let model = get_current_mut_model(s);
                model.offset = 0;

                let users = &get_all_users(s, text, 0);
                let users_email = users
                    .clone()
                    .into_iter()
                    .map(|u| u.email)
                    .collect::<Vec<String>>();

                let users_ids: Vec<usize> = users
                    .clone()
                    .into_iter()
                    .map(|u| u.id as usize)
                    .collect::<Vec<usize>>();

                update_select_component_with_ids(s, "user_list", users_email, users_ids);
            })
            .with_name("searchUser"),
    ));

    layout.add_child(components::selector::select_component_with_ids(
        users_email,
        users_ids,
        "user_list",
        on_select,
    ));

    Dialog::new()
        .title("auth")
        .content(layout)
        .button("refresh", on_refresh)
        .button("previous", on_previous)
        .button("next", on_next)
        .full_screen()
        .with_name(Sidebar::ROLE.to_string())
}

#[tokio::main]
async fn get_all_users(s: &mut Cursive, text: &str, offset: i64) -> Vec<User> {
    let model = get_current_model(s);

    match &model.conn {
        Conn::SQLITE(c) => {
            let query = format!(
                "SELECT * FROM users WHERE email LIKE '%{}%' ORDER BY id LIMIT 25 OFFSET {} ",
                text, offset
            );

            let conn = match c.clone().connection {
                Some(conn) => conn,
                None => panic!("database not connected"),
            };

            let r_out: Result<Vec<User>, sqlx::Error> =
                sqlx::query_as(&query).fetch_all(&conn).await;

            match r_out {
                Ok(u) => u,
                Err(_) => vec![],
            }
        }
        Conn::MYSQL(c) => {
            let query = format!(
                "SELECT * FROM users WHERE email LIKE '%{}%' ORDER BY id LIMIT 25 OFFSET {}",
                text, offset
            );

            let conn = match c.clone().connection {
                Some(conn) => conn,
                None => panic!("database not connected"),
            };

            let r_out: Result<Vec<User>, sqlx::Error> =
                sqlx::query_as(&query).fetch_all(&conn).await;

            match r_out {
                Ok(u) => u,
                Err(_) => vec![],
            }
        }
        Conn::None => vec![],
    }
}

#[tokio::main]
async fn get_total_users_count(s: &mut Cursive, text: &str) -> i64 {
    let model = get_current_model(s);

    match &model.conn {
        Conn::SQLITE(c) => {
            let query = format!(
                "SELECT COUNT(id) AS id FROM users WHERE email LIKE '%{}%'",
                text
            );

            let conn = match c.clone().connection {
                Some(conn) => conn,
                None => panic!("database not connected"),
            };

            let r_out: Result<TotalUsers, sqlx::Error> =
                sqlx::query_as(&query).fetch_one(&conn).await;

            match r_out {
                Ok(u) => u.id,
                Err(_) => 0,
            }
        }
        Conn::MYSQL(c) => {
            let query = format!(
                "SELECT COUNT(id) AS id FROM users WHERE email LIKE '%{}%'",
                text
            );

            let conn = match c.clone().connection {
                Some(conn) => conn,
                None => panic!("database not connected"),
            };

            let r_out: Result<TotalUsers, sqlx::Error> =
                sqlx::query_as(&query).fetch_one(&conn).await;

            match r_out {
                Ok(u) => u.id,
                Err(_) => 0,
            }
        }
        Conn::None => 0,
    }
}

#[tokio::main]
async fn update_user_role(s: &mut Cursive, id: i64, role: String) -> u64 {
    let model = get_current_model(s);

    match &model.conn {
        Conn::SQLITE(c) => {
            let query = "UPDATE users SET role=? WHERE id=?";

            c.execute(
                query,
                vec![ColType::String(Some(role)), ColType::Integer(Some(id))],
            )
            .await
        }
        Conn::MYSQL(c) => {
            let query = "UPDATE users SET role=? WHERE id=?";

            c.execute(
                query,
                vec![ColType::String(Some(role)), ColType::Integer(Some(id))],
            )
            .await
        }
        Conn::None => 0,
    }
}

#[tokio::main]
async fn get_user(s: &mut Cursive, id: i64) -> Option<User> {
    let model = get_current_model(s);

    match &model.conn {
        Conn::SQLITE(c) => {
            let query = "SELECT * FROM users where id=?";

            let conn = match c.clone().connection {
                Some(conn) => conn,
                None => panic!("database not connected"),
            };

            let r_out: Result<User, sqlx::Error> =
                sqlx::query_as(query).bind(id).fetch_one(&conn).await;

            match r_out {
                Ok(u) => Some(u),
                Err(_) => None,
            }
        }
        Conn::MYSQL(c) => {
            let query = "SELECT * FROM users where id=?";

            let conn = match c.clone().connection {
                Some(conn) => conn,
                None => panic!("database not connected"),
            };

            let r_out: Result<User, sqlx::Error> =
                sqlx::query_as(query).bind(id).fetch_one(&conn).await;

            match r_out {
                Ok(u) => Some(u),
                Err(_) => None,
            }
        }
        Conn::None => None,
    }
}

fn edit_role(s: &mut Cursive, idx: usize) {
    let roles = get_current_model(s).rolelist;
    let curr_user = get_user(s, idx as i64);

    let curr_role = match curr_user {
        Some(u) => u.role,
        None => {
            s.add_layer(Dialog::info("user does not exists!"));
            panic!();
        }
    };

    let mut group: RadioGroup<String> = RadioGroup::new();

    let mut list = LinearLayout::new(Orientation::Vertical);
    list.add_child(group.button_str("").with_if(curr_role.is_empty(), |b| {
        b.select();
    }));

    for r in roles {
        list.add_child(group.button_str(r.clone()).with_if(curr_role == r, |b| {
            b.select();
        }));
    }

    let on_submit = move |s: &mut Cursive| {
        let role = group.selection().as_ref().to_owned();

        update_user_role(s, idx as i64, role);

        s.pop_layer();
    };

    let on_cancel = |s: &mut Cursive| {
        s.pop_layer();
    };

    s.add_layer(
        Dialog::new()
            .title("edit role")
            .content(list)
            .button("submit", on_submit)
            .button("cancel", on_cancel),
    );
}
