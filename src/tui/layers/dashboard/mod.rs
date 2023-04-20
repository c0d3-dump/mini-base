use cursive::{
    direction::Orientation,
    view::Nameable,
    views::{Dialog, LinearLayout, NamedView, ResizedView, ScreensView},
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
        "Quit".to_string(), // 4
    ];

    let on_select = |s: &mut Cursive, idx: &usize| {
        if *idx == 4 {
            s.quit();
        }

        let mut dashboards = utils::get_data_from_refname::<
            ScreensView<NamedView<ResizedView<Dialog>>>,
        >(s, "dashboards");
        dashboards.set_active_screen(*idx);
    };

    let sidebar = Dialog::new().content(components::selector::select_component(
        sidebar_items,
        "sidebar_items",
        on_select,
    ));

    let mut dashboards = ScreensView::default();

    dashboards.add_active_screen(role::role_dashboard(s));
    dashboards.add_screen(query::query_dashboard(s));
    dashboards.add_screen(editor::editor_dashboard(s));
    dashboards.add_screen(server::server_dashboard(s));

    s.add_layer(
        LinearLayout::new(Orientation::Horizontal)
            .child(sidebar)
            .child(dashboards.with_name("dashboards")),
    );
}
