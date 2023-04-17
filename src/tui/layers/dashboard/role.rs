use cursive::{
    direction::Orientation,
    view::{Nameable, Resizable},
    views::{Checkbox, Dialog, EditView, LinearLayout, ListView, RadioGroup},
    Cursive, View, With,
};

use crate::tui::{
    components,
    model::{Model, RoleAccess, RoleList, Sidebar},
    utils::{
        get_current_model, get_current_mut_model, get_data_from_refname, update_role_with_model,
    },
};

pub fn role_dashboard(s: &mut Cursive) -> impl View {
    let model = get_current_model(s);

    let role_list_items = model
        .rolelist
        .into_iter()
        .map(|r| r.label)
        .collect::<Vec<String>>();

    let on_select = |s: &mut Cursive, idx: &usize| {
        edit_role(s, *idx);
    };

    let role_list = components::selector::select_component(role_list_items, "role_list", on_select);

    let on_add_role = |s: &mut Cursive| add_role(s);

    Dialog::new()
        .title("role")
        .content(role_list)
        .button("Add Role", on_add_role)
        .full_screen()
        .with_name(Sidebar::ROLE.to_string())
}

fn edit_role(s: &mut Cursive, idx: usize) {
    let role = get_current_model(s).rolelist.get(idx).unwrap().to_owned();

    let mut list = ListView::new();
    list.add_child(
        "label",
        EditView::new().content(role.label).with_name("edit_label"),
    );

    let mut boolean_group: RadioGroup<bool> = RadioGroup::new();
    list.add_child(
        "approval required",
        LinearLayout::new(Orientation::Horizontal)
            .child(boolean_group.button(false, "false"))
            .child(
                boolean_group
                    .button(true, "true")
                    .with_if(role.approval_required, |b| {
                        b.select();
                    }),
            ),
    );

    let all_roles = vec![
        RoleAccess::READ,
        RoleAccess::CREATE,
        RoleAccess::DELETE,
        RoleAccess::UPDATE,
    ];
    let mut role_access_list = vec![];
    for ra in all_roles {
        if role.role_access.contains(&ra) {
            role_access_list.push((ra.to_string(), true));
        } else {
            role_access_list.push((ra.to_string(), false));
        }
    }

    let check_box =
        components::checkbox_group::checkbox_group_component("role_access", role_access_list);
    list.add_child("role access", check_box);

    let on_submit = move |s: &mut Cursive| {
        let all_roles = vec![
            RoleAccess::READ,
            RoleAccess::CREATE,
            RoleAccess::DELETE,
            RoleAccess::UPDATE,
        ];

        let roleaccess = get_checked_role_access_data(s, all_roles);

        let label = s
            .call_on_name("edit_label", |view: &mut EditView| view.get_content())
            .unwrap()
            .to_string();

        let rolelist = RoleList {
            label: label.to_string(),
            approval_required: *boolean_group.selection(),
            role_access: roleaccess,
        };

        let model = get_current_mut_model(s);
        model.rolelist[idx] = rolelist;

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
            m.rolelist.push(RoleList {
                label: data.get_content().to_string(),
                approval_required: false,
                role_access: vec![],
            });
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

pub fn get_checked_role_access_data(
    s: &mut Cursive,
    all_items: Vec<RoleAccess>,
) -> Vec<RoleAccess> {
    let mut checked_items = vec![];

    for i in all_items {
        let checkbox = s.find_name::<Checkbox>(&i.to_string()).unwrap();
        if checkbox.is_checked() {
            checked_items.push(i);
        }
    }

    checked_items
}
