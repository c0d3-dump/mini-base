use cursive::{
    direction::Orientation,
    view::{Nameable, Resizable},
    views::{Dialog, EditView, LinearLayout, ListView, NamedView, RadioGroup, ResizedView},
    Cursive, With,
};

use crate::{
    queries::model::Role,
    tui::{
        components::{
            self,
            selector::{add_select_item, remove_select_item, update_select_item},
        },
        model::Sidebar,
        utils::{get_current_mut_model, get_data_from_refname},
    },
};

pub fn role_dashboard(s: &mut Cursive) -> NamedView<ResizedView<Dialog>> {
    let model = get_current_mut_model(s);

    let on_select = |s: &mut Cursive, idx: &usize| {
        edit_role(s, *idx);
    };

    let optional_roles = futures::executor::block_on(model.get_all_roles());

    let mut roles = vec![];

    match optional_roles {
        Ok(r) => {
            roles = r;
        }
        Err(e) => s.add_layer(Dialog::info(e)),
    }

    let role_list = components::selector::select_component(
        roles.into_iter().map(|r| (r.id as usize, r.name)).collect(),
        "role_list",
        on_select,
    );

    Dialog::new()
        .title("Role")
        .content(role_list)
        .button("Add Role", add_role)
        .full_screen()
        .with_name(Sidebar::ROLE.to_string())
}

fn edit_role(s: &mut Cursive, idx: usize) {
    let model = get_current_mut_model(s);

    let optional_role = futures::executor::block_on(model.get_role_by_id(idx as i64));
    let role;

    match optional_role {
        Ok(r) => {
            role = r;
        }
        Err(e) => {
            s.add_layer(Dialog::info(e));
            return;
        }
    }

    let mut list = ListView::new();
    list.add_child(
        "label",
        EditView::new().content(role.name).with_name("edit_label"),
    );

    let mut boolean_group: RadioGroup<bool> = RadioGroup::new();
    list.add_child(
        "default role",
        LinearLayout::new(Orientation::Horizontal)
            .child(boolean_group.button(false, "False"))
            .child(
                boolean_group
                    .button(true, "True")
                    .with_if(role.is_default, |b| {
                        b.select();
                    }),
            ),
    );

    let storage_list = vec![
        ("Read".to_string(), role.can_read),
        ("Write".to_string(), role.can_write),
        ("Delete".to_string(), role.can_delete),
    ];

    let check_box =
        components::checkbox_group::checkbox_group_component("storage_access", storage_list);

    list.add_child("storage access", check_box);

    let on_submit = move |s: &mut Cursive| {
        let edit_ref = s.find_name::<EditView>("edit_label").unwrap();
        let label = edit_ref.get_content().to_string();

        let storageaccess = components::checkbox_group::get_checked_data(
            s,
            vec![
                "Read".to_string(),
                "Write".to_string(),
                "Delete".to_string(),
            ],
        );

        let role = Role {
            id: idx as i64,
            name: label.clone(),
            is_default: *boolean_group.selection(),
            can_read: storageaccess[0],
            can_write: storageaccess[1],
            can_delete: storageaccess[2],
        };

        let model = get_current_mut_model(s);
        if *boolean_group.selection() {
            let res = futures::executor::block_on(model.unset_default_role());

            match res {
                Ok(_) => {}
                Err(e) => {
                    s.add_layer(Dialog::info(e));
                    return;
                }
            };
        }

        let res = futures::executor::block_on(model.edit_role(role));
        match res {
            Ok(_) => {}
            Err(e) => {
                s.add_layer(Dialog::info(e));
                return;
            }
        };

        update_select_item(s, "role_list", label, idx);
        s.pop_layer();
    };

    let on_delete = move |s: &mut Cursive| {
        let model = get_current_mut_model(s);
        let res = futures::executor::block_on(model.delete_role(idx as i64));

        match res {
            Ok(_) => {}
            Err(e) => {
                s.add_layer(Dialog::info(e));
                return;
            }
        };

        remove_select_item(s, "role_list", idx);

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
            .button("delete", on_delete)
            .button("cancel", on_cancel),
    );
}

fn add_role(s: &mut Cursive) {
    let on_submit = |s: &mut Cursive| {
        let data = get_data_from_refname::<EditView>(s, "add_role_text");

        let model = get_current_mut_model(s);

        let res = futures::executor::block_on(model.add_new_role(data.get_content().to_string()));

        match res {
            Ok(i) => {
                add_select_item(s, "role_list", data.get_content().to_string(), i as usize);

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
            .title("add role")
            .content(textedit.with_name("add_role_text"))
            .button("submit", on_submit)
            .button("cancel", on_cancel),
    );
}
