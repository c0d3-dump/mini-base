use cursive::{
    view::{Nameable, Scrollable},
    views::{Dialog, EditView, NamedView},
    Cursive,
};

use crate::tui::{
    components::{self, selector::add_select_item},
    model::Sidebar,
    utils::{get_current_mut_model, get_data_from_refname},
};

pub fn migration_dashboard(s: &mut Cursive) -> NamedView<Dialog> {
    let on_up = |_s: &mut Cursive| {};

    let on_down = |_s: &mut Cursive| {};

    let model = get_current_mut_model(s);

    let on_select = |s: &mut Cursive, idx: &usize| {
        // edit_role(s, *idx);
    };

    let optional_migrations = futures::executor::block_on(model.get_all_migrations());

    let mut migrations = vec![];

    match optional_migrations {
        Ok(m) => {
            migrations = m;
        }
        Err(e) => s.add_layer(Dialog::info(e)),
    }

    let migration_list = components::selector::select_component(
        migrations
            .into_iter()
            .map(|m| (m.id as usize, m.name))
            .collect(),
        "migration_list",
        on_select,
    );

    Dialog::new()
        .title("Migration")
        .content(migration_list.with_name("migration").scrollable())
        .padding_lrtb(1, 1, 1, 0)
        .button("up", on_up)
        .button("down", on_down)
        .button("add migration", add_migration)
        .with_name(Sidebar::Migration.to_string())
}

fn add_migration(s: &mut Cursive) {
    let on_submit = |s: &mut Cursive| {
        let migration_ref = get_data_from_refname::<EditView>(s, "add_migration_text");
        let migration_text = migration_ref.get_content().to_string();

        let model = get_current_mut_model(s);
        let res = futures::executor::block_on(model.add_new_migration(migration_text.clone()));

        match res {
            Ok(i) => {
                add_select_item(s, "migration_list", migration_text, i as usize);

                s.pop_layer();
            }
            Err(e) => {
                s.add_layer(Dialog::info(e));
            }
        }
    };

    let on_cancel = |s: &mut Cursive| {
        s.pop_layer();
    };

    let textedit = EditView::new();

    s.add_layer(
        Dialog::new()
            .title("Add Migration Name")
            .padding_lrtb(1, 1, 1, 0)
            .content(textedit.with_name("add_migration_text"))
            .button("submit", on_submit)
            .button("cancel", on_cancel),
    );
}
