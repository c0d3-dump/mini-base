use cursive::{
    direction::Orientation,
    view::Nameable,
    views::{Dialog, LinearLayout, StackView},
    Cursive,
};

use crate::tui::{components, model::Sidebar, utils};

pub mod editor;
pub mod query;
pub mod role;
pub mod server;

pub fn display_dashboard(s: &mut Cursive) {
    let sidebar_items = vec![
        Sidebar::ROLE.to_string(),
        Sidebar::QUERY.to_string(),
        Sidebar::EDITOR.to_string(),
        Sidebar::SERVER.to_string(),
    ];

    let on_select = |s: &mut Cursive, idx: &usize| {
        let mut dashboards = utils::get_data_from_refname::<StackView>(s, "dashboards");

        let sidebar_items = vec![
            Sidebar::ROLE.to_string(),
            Sidebar::QUERY.to_string(),
            Sidebar::EDITOR.to_string(),
            Sidebar::SERVER.to_string(),
        ];

        let layerpos = dashboards
            .find_layer_from_name(sidebar_items.get(*idx).unwrap())
            .unwrap();
        dashboards.move_to_front(layerpos);
    };

    let sidebar = Dialog::new().content(components::selector::select_component(
        sidebar_items,
        "sidebar_items",
        on_select,
    ));

    let mut dashboards = StackView::default();

    dashboards.add_fullscreen_layer(editor::editor_dashboard(s));
    dashboards.add_fullscreen_layer(query::query_dashboard(s));
    dashboards.add_fullscreen_layer(role::role_dashboard(s));
    dashboards.add_fullscreen_layer(server::server_dashboard(s));

    s.add_layer(
        LinearLayout::new(Orientation::Horizontal)
            .child(sidebar)
            .child(dashboards.with_name("dashboards")),
    );
}
