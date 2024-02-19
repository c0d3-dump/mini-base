use std::time::Duration;

use cursive::{
    direction::Orientation,
    view::{Nameable, Resizable},
    views::{Dialog, LinearLayout, NamedView, ResizedView, ScreensView},
    Cursive,
};
use enum_iterator::all;

use crate::tui::{
    components,
    model::Sidebar,
    utils::{self, get_current_mut_model},
};

pub mod apis;
pub mod config;
pub mod migration;
pub mod query;
pub mod role;
pub mod users;

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
                if sidebar == Sidebar::Quit {
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
                        ScreensView<ResizedView<NamedView<Dialog>>>,
                    >(s, "dashboards");

                    dashboards.set_active_screen(*idx);
                }
            }
            None => panic!("error: {}", idx),
        }
    };

    let sidebar = Dialog::new()
        .content(components::selector::select_component(
            sidebar_items,
            "sidebar_items",
            on_select,
        ))
        .padding_lrtb(1, 1, 1, 0);

    let mut dashboards = ScreensView::default();

    dashboards.add_active_screen(config::config_dashboard(s).full_screen());
    dashboards.add_screen(role::role_dashboard(s).full_screen());
    dashboards.add_screen(users::users_dashboard(s).full_screen());
    dashboards.add_screen(query::query_dashboard(s).full_screen());
    dashboards.add_screen(apis::apis_dashboard(s).full_screen());
    dashboards.add_screen(migration::migration_dashboard(s).full_screen());

    s.add_layer(
        LinearLayout::new(Orientation::Horizontal)
            .child(sidebar)
            .child(dashboards.with_name("dashboards")),
    );
}
