use cursive::{
    direction::Orientation,
    view::{Nameable, Resizable},
    views::{Dialog, EditView, LinearLayout, ListView, NamedView, RadioGroup, ResizedView},
    Cursive, With,
};

use crate::tui::{
    components,
    model::{Model, Sidebar, StorageAccess},
    utils::{
        get_current_model, get_current_mut_model, get_data_from_refname, update_role_with_model,
    },
};

pub fn role_dashboard(s: &mut Cursive) -> NamedView<ResizedView<Dialog>> {
    let model = get_current_model(s);

    let on_select = |s: &mut Cursive, idx: &usize| {
        edit_role(s, *idx);
    };

    let role_list = components::selector::select_component(model.rolelist, "role_list", on_select);

    let on_add_role = add_role;

    Dialog::new()
        .title("role")
        .content(role_list)
        .button("Add Role", on_add_role)
        .full_screen()
        .with_name(Sidebar::ROLE.to_string())
}

fn edit_role(s: &mut Cursive, idx: usize) {
    let model = get_current_model(s);
    let role = model.rolelist.get(idx).unwrap().to_owned();
    let default_role = model.default_role;
    let storagelist = model.storage_access.get(&role).unwrap_or(&StorageAccess {
        delete: false,
        read: false,
        write: false,
    });

    let mut list = ListView::new();
    list.add_child(
        "label",
        EditView::new()
            .content(role.clone())
            .with_name("edit_label"),
    );

    let mut boolean_group: RadioGroup<bool> = RadioGroup::new();
    list.add_child(
        "default role",
        LinearLayout::new(Orientation::Horizontal)
            .child(boolean_group.button(false, "False"))
            .child(
                boolean_group
                    .button(true, "True")
                    .with_if(default_role == role, |b| {
                        b.select();
                    }),
            ),
    );

    let storage_list = vec![
        ("Read".to_string(), storagelist.read),
        ("Write".to_string(), storagelist.write),
        ("Delete".to_string(), storagelist.delete),
    ];

    let check_box =
        components::checkbox_group::checkbox_group_component("storage_access", storage_list);

    list.add_child("storage access", check_box);

    let on_submit = move |s: &mut Cursive| {
        let label = s
            .call_on_name("edit_label", |view: &mut EditView| view.get_content())
            .unwrap()
            .to_string();

        let storageaccess = components::checkbox_group::get_checked_data(
            s,
            vec![
                "Read".to_string(),
                "Write".to_string(),
                "Delete".to_string(),
            ],
        );

        let model = get_current_mut_model(s);
        model.rolelist[idx] = label.clone();
        if *boolean_group.selection() == true {
            model.default_role = label.clone();
        } else if model.default_role == label {
            model.default_role = "".to_string();
        }

        model.storage_access.insert(
            role.to_string(),
            StorageAccess {
                read: storageaccess.contains(&"Read".to_string()),
                write: storageaccess.contains(&"Write".to_string()),
                delete: storageaccess.contains(&"Delete".to_string()),
            },
        );

        update_role_with_model(s);

        s.pop_layer();
    };

    let on_delete = move |s: &mut Cursive| {
        let model = get_current_mut_model(s);
        model.rolelist.remove(idx);

        update_role_with_model(s);

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

        s.with_user_data(|m: &mut Model| {
            m.rolelist.push(data.get_content().to_string());
        })
        .unwrap();

        update_role_with_model(s);

        s.pop_layer();
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
