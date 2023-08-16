use std::time::Duration;

use cursive::{
    direction::Orientation,
    view::Nameable,
    views::{Dialog, LinearLayout, NamedView, ResizedView, ScreensView},
    Cursive,
};
use enum_iterator::all;

use crate::tui::{
    components,
    model::Sidebar,
    utils::{self, get_current_mut_model},
};

// pub mod auth;
// pub mod editor;
pub mod query;
pub mod role;
// pub mod server;

pub fn display_dashboard(s: &mut Cursive) {
    let sidebar_items = all::<Sidebar>()
        .enumerate()
        .map(|(idx, item)| (idx, item.to_string()))
        .collect::<Vec<(usize, String)>>();

    let on_select = |s: &mut Cursive, idx: &usize| {
        let optional_sidebar = all::<Sidebar>()
            .enumerate()
            .filter(|(i, _)| i == idx)
            .map(|(_, x)| x)
            .next();

        match optional_sidebar {
            Some(sidebar) => {
                if sidebar == Sidebar::QUIT {
                    let model = get_current_mut_model(s);

                    match &model.handle {
                        Some(h) => h.graceful_shutdown(Some(Duration::from_secs(3))),
                        None => {}
                    }

                    match &model.conn {
                        Some(c) => {
                            futures::executor::block_on(c.close());
                        }
                        None => {}
                    }

                    s.quit();
                } else {
                    let mut dashboards = utils::get_data_from_refname::<
                        ScreensView<NamedView<ResizedView<Dialog>>>,
                    >(s, "dashboards");

                    dashboards.set_active_screen(*idx);
                }
            }
            None => panic!("error: {}", idx),
        }
    };

    let sidebar = Dialog::new().content(components::selector::select_component(
        sidebar_items,
        "sidebar_items",
        on_select,
    ));

    let mut dashboards = ScreensView::default();

    dashboards.add_active_screen(role::role_dashboard(s));
    dashboards.add_screen(query::query_dashboard(s));
    // dashboards.add_screen(auth::auth_dashboard(s));
    // dashboards.add_screen(editor::editor_dashboard(s));
    // dashboards.add_screen(server::server_dashboard(s));

    s.add_layer(
        LinearLayout::new(Orientation::Horizontal)
            .child(sidebar)
            .child(dashboards.with_name("dashboards")),
    );
}
