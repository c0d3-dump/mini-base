use cursive::{
    view::{Nameable, Resizable, Scrollable},
    views::{Dialog, ListView, NamedView, ResizedView},
    Cursive,
};

use crate::tui::model::Sidebar;

pub fn migration_dashboard(_s: &mut Cursive) -> NamedView<ResizedView<Dialog>> {
    let list = ListView::new();

    let on_up = |_s: &mut Cursive| {};

    let on_down = |_s: &mut Cursive| {};

    let on_add_migration = |_s: &mut Cursive| {};

    Dialog::new()
        .title("Migration")
        .content(list.with_name("migration").scrollable())
        .padding_lrtb(1, 1, 1, 0)
        .button("up", on_up)
        .button("down", on_down)
        .button("add migration", on_add_migration)
        .full_screen()
        .with_name(Sidebar::Migration.to_string())
}
