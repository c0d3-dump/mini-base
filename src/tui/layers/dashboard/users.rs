use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, EditView, NamedView, ResizedView, SelectView, TextView},
    Cursive,
};

use crate::tui::{
    components::{
        self,
        selector::{add_select_item, remove_select_item},
    },
    model::Sidebar,
    utils::{get_current_mut_model, get_data_from_refname},
};

pub fn users_dashboard(s: &mut Cursive) -> NamedView<ResizedView<Dialog>> {
    let model = get_current_mut_model(s);

    let on_select = |s: &mut Cursive, idx: &usize| {
        edit_user(s, *idx);
    };

    let optional_users = futures::executor::block_on(model.get_all_users());

    let mut users = vec![];

    match optional_users {
        Ok(u) => {
            users = u;
        }
        Err(e) => s.add_layer(Dialog::info(e)),
    }

    let user_list = components::selector::select_component(
        users
            .into_iter()
            .map(|u| (u.id as usize, u.email))
            .collect(),
        "user_list",
        on_select,
    );

    Dialog::new()
        .title("Users")
        .content(user_list)
        .padding_lrtb(1, 1, 1, 0)
        .button("Add User", add_user)
        .full_screen()
        .with_name(Sidebar::USERS.to_string())
}

fn edit_user(s: &mut Cursive, idx: usize) {
    let model = get_current_mut_model(s);

    let optional_role_access =
        futures::executor::block_on(model.get_user_role_access_by_id(idx as i64));
    let role_access;

    match optional_role_access {
        Ok(r) => {
            role_access = r;
        }
        Err(e) => {
            s.add_layer(Dialog::info(e));
            return;
        }
    }

    let on_select = |s: &mut Cursive, idx: &usize| {
        let model = get_current_mut_model(s);
        model.temp.selected_role_access_id = Some(*idx as i64);

        let select_ref = get_data_from_refname::<SelectView<usize>>(s, "role_access_list");
        let optional_role_name = select_ref.iter().find(|(_, sr)| *sr == idx);

        match optional_role_name {
            Some((role_name, _)) => {
                let mut dialog_ref = get_data_from_refname::<Dialog>(s, "user_access_role");
                dialog_ref.set_title(role_name);
            }
            None => {}
        }
    };

    let list = components::selector::select_component(
        role_access
            .iter()
            .map(|ra| (ra.role_id as usize, ra.name.clone()))
            .collect(),
        "role_access_list",
        on_select,
    );

    let on_submit = move |s: &mut Cursive| {
        let model = get_current_mut_model(s);

        match model.temp.selected_role_access_id {
            Some(i) => {
                let res = futures::executor::block_on(model.update_user_role(idx as i64, i));
                match res {
                    Ok(_) => {}
                    Err(e) => {
                        s.add_layer(Dialog::info(e));
                        return;
                    }
                }
            }
            None => {}
        }

        let model = get_current_mut_model(s);
        model.temp.selected_role_access_id = None;

        s.pop_layer();
    };

    let on_delete = move |s: &mut Cursive| {
        s.add_layer(
            Dialog::new()
                .content(TextView::new(
                    "Are you sure you want to remove remove user role?",
                ))
                .button("cancel", |s: &mut Cursive| {
                    s.pop_layer();
                })
                .button("continue", move |s: &mut Cursive| {
                    let model = get_current_mut_model(s);
                    model.temp.selected_role_access_id = None;

                    let res = futures::executor::block_on(model.remove_default_user(idx as i64));
                    match res {
                        Ok(_) => {}
                        Err(e) => {
                            s.add_layer(Dialog::info(e));
                            return;
                        }
                    };

                    remove_select_item(s, "user_list", idx);
                    s.pop_layer();
                    s.pop_layer();
                }),
        );
    };

    let on_cancel = |s: &mut Cursive| {
        let model = get_current_mut_model(s);
        model.temp.selected_role_access_id = None;

        s.pop_layer();
    };

    s.add_layer(
        Dialog::new()
            .title("")
            .content(list)
            .padding_lrtb(1, 1, 1, 0)
            .button("submit", on_submit)
            .button("delete", on_delete)
            .button("cancel", on_cancel)
            .with_name("user_access_role"),
    );

    let mut role_access_ref = get_data_from_refname::<SelectView<usize>>(s, "role_access_list");
    let selected_role_access_id = role_access
        .iter()
        .enumerate()
        .find(|(_, ra)| ra.is_selected == true);

    match selected_role_access_id {
        Some((i, _)) => {
            role_access_ref.set_selection(i);
        }
        None => {}
    }

    let mut dialog_ref = get_data_from_refname::<Dialog>(s, "user_access_role");
    match selected_role_access_id {
        Some((_, n)) => {
            dialog_ref.set_title(n.name.to_string());

            let model = get_current_mut_model(s);
            model.temp.selected_role_access_id = Some(n.role_id);
        }
        None => {}
    }
}

fn add_user(s: &mut Cursive) {
    let on_submit = |s: &mut Cursive| {
        let user_ref = get_data_from_refname::<EditView>(s, "add_user_text");
        let user_text = user_ref.get_content().to_string();

        let model = get_current_mut_model(s);
        let res = futures::executor::block_on(model.add_default_user(user_text.clone()));

        match res {
            Ok(i) => {
                add_select_item(s, "user_list", user_text, i as usize);
                s.pop_layer();
            }
            Err(e) => {
                s.add_layer(Dialog::info(e));
                return;
            }
        };
    };

    let on_cancel = |s: &mut Cursive| {
        s.pop_layer();
    };

    let textedit = EditView::new();

    s.add_layer(
        Dialog::new()
            .title("Add User Email")
            .padding_lrtb(1, 1, 1, 0)
            .content(textedit.with_name("add_user_text"))
            .button("submit", on_submit)
            .button("cancel", on_cancel),
    );
}
