use std::rc::Rc;

use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, NamedView, ResizedView, TextArea},
    Cursive,
};

use crate::tui::{
    components,
    model::Sidebar,
    utils::{
        get_current_model, get_current_mut_model, get_data_from_refname, update_query_with_model,
    },
};

pub fn editor_dashboard(s: &mut Cursive) -> NamedView<ResizedView<Dialog>> {
    let model = get_current_model(s);
    let query_list_items = model
        .querylist
        .into_iter()
        .map(|r| r.label)
        .collect::<Vec<String>>();

    let on_select = |s: &mut Cursive, idx: &usize| {
        let i = Rc::new(idx.to_owned());
        let query = get_current_model(s).querylist.get(*i).unwrap().to_owned();

        let on_submit = move |s: &mut Cursive| {
            let data = get_data_from_refname::<TextArea>(s, "query_editor");

            let model = get_current_mut_model(s);
            model.querylist[*i].query = data.get_content().to_string();

            update_query_with_model(s);

            s.pop_layer();
        };

        s.add_layer(components::editor::editor_componant(
            "query_editor".to_string(),
            "editor",
            on_submit,
            query.query,
        ));
    };

    let query_list =
        components::selector::select_component(query_list_items, "query_editor_list", on_select);

    Dialog::new()
        .title("editor")
        .content(query_list)
        .full_screen()
        .with_name(Sidebar::EDITOR.to_string())
}
