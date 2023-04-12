use std::collections::HashMap;

use cursive::{
    view::Resizable,
    views::{Dialog, ScreensView, StackView, TextArea, TextView},
    Cursive, ScreenId,
};

mod components;
mod model;
mod style;
mod utils;

pub fn run() {
    let mut app = cursive::default();

    app.set_theme(style::get_theme());

    let mut dmodel = model::Model::default();
    dmodel.dbtype = vec![
        String::from("sqlite"),
        String::from("mysql"),
        String::from("postgres"),
    ];

    app.set_user_data(dmodel);

    let on_select = |s: &mut Cursive, idx: &usize| {
        let model = utils::get_current_model(s);
        let data = model.dbtype.get(*idx).unwrap();

        let items = vec![String::from("lol"), String::from("b2b")];

        let on_select = |s: &mut Cursive, idx: &usize| {};

        let list = components::selector::select_component(s, items, on_select);

        s.add_layer(Dialog::new().title("list").content(list.full_screen()));
    };

    let model = utils::get_current_model(&mut app);
    let select = components::selector::select_component(&mut app, model.dbtype, on_select);

    let on_editor = |s: &mut Cursive| {
        let data = utils::get_data_from_refname::<TextArea>(s, "editor");

        s.add_layer(Dialog::info(data.get_content().to_string()));
    };

    app.add_layer(
        Dialog::new()
            .title("mini base")
            .content(select)
            .button("next", move |s| {
                let editor = components::editor::editor_componant(
                    s,
                    String::from("editor"),
                    "sql",
                    on_editor,
                );

                s.add_layer(editor);
            })
            .button("quit", |s| {
                s.quit();
            }),
    );

    app.run();
}
