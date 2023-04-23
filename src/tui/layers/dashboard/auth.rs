use cursive::{
    direction::Orientation,
    view::{Nameable, Resizable},
    views::{Dialog, LinearLayout, NamedView, RadioGroup, ResizedView},
    Cursive, With,
};

use crate::{
    database::model::ColType,
    server::model::User,
    tui::{
        components::{
            self,
            selector::{update_select_component, update_select_component_with_ids},
        },
        model::{Conn, Sidebar},
        utils::{get_current_model, get_data_from_refname},
    },
};

pub fn auth_dashboard(s: &mut Cursive) -> NamedView<ResizedView<Dialog>> {
    let users = get_all_users(s);

    let users_name = users
        .clone()
        .into_iter()
        .map(|u| u.email)
        .collect::<Vec<String>>();

    let on_select = move |s: &mut Cursive, idx: &usize| {
        edit_role(s, *idx);
    };

    let on_refresh = |s: &mut Cursive| {
        let users = &get_all_users(s);
        let users_name = users
            .clone()
            .into_iter()
            .map(|u| u.email)
            .collect::<Vec<String>>();

        let users_ids: Vec<usize> = users
            .clone()
            .into_iter()
            .map(|u| u.id as usize)
            .collect::<Vec<usize>>();

        update_select_component_with_ids(s, "user_list", users_name, users_ids);
    };

    let users_ids: Vec<usize> = users
        .clone()
        .into_iter()
        .map(|u| u.id as usize)
        .collect::<Vec<usize>>();

    let user_list = components::selector::select_component_with_ids(
        users_name,
        users_ids,
        "user_list",
        on_select,
    );

    Dialog::new()
        .title("auth")
        .content(user_list)
        .button("refresh", on_refresh)
        .full_screen()
        .with_name(Sidebar::ROLE.to_string())
}

#[tokio::main]
async fn get_all_users(s: &mut Cursive) -> Vec<User> {
    let model = get_current_model(s);

    match &model.conn {
        Conn::SQLITE(c) => {
            let query = "SELECT * FROM users";

            let conn = match c.clone().connection {
                Some(conn) => conn,
                None => panic!("database not connected"),
            };

            let r_out: Result<Vec<User>, sqlx::Error> =
                sqlx::query_as(query).fetch_all(&conn).await;

            match r_out {
                Ok(u) => u,
                Err(_) => vec![],
            }
        }
        Conn::MYSQL(c) => {
            let query = "SELECT * FROM users";

            let conn = match c.clone().connection {
                Some(conn) => conn,
                None => panic!("database not connected"),
            };

            let r_out: Result<Vec<User>, sqlx::Error> =
                sqlx::query_as(query).fetch_all(&conn).await;

            match r_out {
                Ok(u) => u,
                Err(_) => vec![],
            }
        }
        Conn::None => vec![],
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
