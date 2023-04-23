use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, EditView, ListView, NamedView, ResizedView},
    Cursive,
};

use crate::tui::{
    components,
    model::{Model, Sidebar},
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
    let role = get_current_model(s).rolelist.get(idx).unwrap().to_owned();

    let mut list = ListView::new();
    list.add_child(
        "label",
        EditView::new().content(role).with_name("edit_label"),
    );

    let on_submit = move |s: &mut Cursive| {
        let label = s
            .call_on_name("edit_label", |view: &mut EditView| view.get_content())
            .unwrap()
            .to_string();

        let model = get_current_mut_model(s);
        model.rolelist[idx] = label;

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
