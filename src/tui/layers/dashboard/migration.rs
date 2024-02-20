use cursive::{
    view::{Nameable, Scrollable},
    views::{Button, Dialog, EditView, ListView, NamedView, TextArea, TextView},
    Cursive,
};

use crate::{
    queries::model::Migration,
    tui::{
        components::{
            self,
            selector::{add_select_item, get_select_item, remove_select_item, update_select_item},
        },
        model::Sidebar,
        utils::{get_current_model, get_current_mut_model, get_data_from_refname},
    },
};

pub fn migration_dashboard(s: &mut Cursive) -> NamedView<Dialog> {
    let on_up = |s: &mut Cursive| {
        let model = get_current_model(s);

        let optional_migrations = futures::executor::block_on(model.get_up_migrations());

        match optional_migrations {
            Ok(migrations) => {
                for migration in migrations {
                    let res = futures::executor::block_on(model.run_migration(migration.up_query));

                    match res {
                        Ok(_) => {
                            if futures::executor::block_on(
                                model.update_executed_migration(migration.id, true),
                            )
                            .is_ok()
                            {
                                let mut item_string =
                                    get_select_item(s, "migration_list", migration.id as usize);

                                if !item_string.contains("(up)") {
                                    item_string += "(up)";
                                }

                                update_select_item(
                                    s,
                                    "migration_list",
                                    item_string,
                                    migration.id as usize,
                                );
                            }
                        }
                        Err(e) => {
                            s.add_layer(Dialog::info(e));
                            return;
                        }
                    }
                }
            }
            Err(e) => s.add_layer(Dialog::info(e)),
        }
    };

    let on_down = |s: &mut Cursive| {
        let model = get_current_model(s);

        let optional_migrations = futures::executor::block_on(model.get_down_migrations());

        match optional_migrations {
            Ok(migrations) => {
                for migration in migrations {
                    let res =
                        futures::executor::block_on(model.run_migration(migration.down_query));

                    match res {
                        Ok(_) => {
                            if futures::executor::block_on(
                                model.update_executed_migration(migration.id, false),
                            )
                            .is_ok()
                            {
                                let mut item_string =
                                    get_select_item(s, "migration_list", migration.id as usize);

                                item_string = item_string.replace("(up)", "");

                                update_select_item(
                                    s,
                                    "migration_list",
                                    item_string,
                                    migration.id as usize,
                                );
                            }
                        }
                        Err(e) => {
                            s.add_layer(Dialog::info(e));
                            return;
                        }
                    }
                }
            }
            Err(e) => s.add_layer(Dialog::info(e)),
        }
    };

    let on_select = |s: &mut Cursive, idx: &usize| {
        edit_migration(s, *idx);
    };

    let model = get_current_mut_model(s);

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
            .map(|m| {
                if m.executed {
                    (m.id as usize, m.name + "(up)")
                } else {
                    (m.id as usize, m.name)
                }
            })
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

fn edit_migration(s: &mut Cursive, idx: usize) {
    let model = get_current_model(s);

    let optional_migration =
        futures::executor::block_on(model.get_migration_name_by_id(idx as i64));
    let migration = match optional_migration {
        Ok(m) => m,
        Err(e) => {
            s.add_layer(Dialog::info(e));
            return;
        }
    };

    let mut list = ListView::new();

    list.add_child(
        "Up Migration",
        Button::new("", move |s: &mut Cursive| {
            let model = get_current_mut_model(s);

            let mut up_migration_string: String = model.temp.up_migration_string.clone();

            if !model.temp.up_migration_written {
                let optional_up_migration_string =
                    futures::executor::block_on(model.get_up_migration_by_id(idx as i64));

                match optional_up_migration_string {
                    Ok(s) => {
                        up_migration_string = s.up_query;
                        model.temp.up_migration_written = true;
                        model.temp.up_migration_string = up_migration_string.clone();
                    }
                    Err(e) => {
                        s.add_layer(Dialog::info(e));
                        return;
                    }
                }
            }

            let on_submit = move |s: &mut Cursive| {
                let query_ref = get_data_from_refname::<TextArea>(s, "up_migration_editor");

                let model = get_current_mut_model(s);
                model.temp.up_migration_string = query_ref.get_content().to_string();

                s.pop_layer();
            };

            s.add_layer(components::editor::editor_componant(
                "up_migration_editor",
                "Editor",
                on_submit,
                up_migration_string,
            ));
        }),
    );

    list.add_child(
        "Down Migration",
        Button::new("", move |s: &mut Cursive| {
            let model = get_current_mut_model(s);

            let mut down_migration_string: String = model.temp.down_migration_string.clone();

            if !model.temp.down_migration_written {
                let optional_down_migration_string =
                    futures::executor::block_on(model.get_down_migration_by_id(idx as i64));

                match optional_down_migration_string {
                    Ok(s) => {
                        down_migration_string = s.down_query;
                        model.temp.down_migration_written = true;
                        model.temp.down_migration_string = down_migration_string.clone();
                    }
                    Err(e) => {
                        s.add_layer(Dialog::info(e));
                        return;
                    }
                }
            }

            let on_submit = move |s: &mut Cursive| {
                let query_ref = get_data_from_refname::<TextArea>(s, "down_migration_editor");

                let model = get_current_mut_model(s);
                model.temp.down_migration_string = query_ref.get_content().to_string();

                s.pop_layer();
            };

            s.add_layer(components::editor::editor_componant(
                "down_migration_editor",
                "Editor",
                on_submit,
                down_migration_string,
            ));
        }),
    );

    let on_submit = move |s: &mut Cursive| {
        let model = get_current_mut_model(s);

        if model.temp.up_migration_written {
            model.temp.up_migration_written = false;

            let up_migration_string = model.temp.up_migration_string.clone();
            model.temp.up_migration_string.clear();

            let res = futures::executor::block_on(model.edit_up_migration(Migration {
                id: idx as i64,
                name: "".to_string(),
                up_query: up_migration_string,
                down_query: "".to_string(),
            }));

            match res {
                Ok(_) => {}
                Err(e) => {
                    s.add_layer(Dialog::info(e));
                    return;
                }
            }
        }

        if model.temp.down_migration_written {
            model.temp.down_migration_written = false;

            let down_migration_string = model.temp.down_migration_string.clone();
            model.temp.down_migration_string.clear();

            let res = futures::executor::block_on(model.edit_down_migration(Migration {
                id: idx as i64,
                name: "".to_string(),
                up_query: "".to_string(),
                down_query: down_migration_string,
            }));

            match res {
                Ok(_) => {}
                Err(e) => {
                    s.add_layer(Dialog::info(e));
                    return;
                }
            }
        }

        s.pop_layer();
    };

    let on_delete = move |s: &mut Cursive| {
        s.add_layer(
            Dialog::new()
                .content(TextView::new(
                    "Are you sure you want to remove remove migration?",
                ))
                .button("cancel", |s: &mut Cursive| {
                    s.pop_layer();
                })
                .button("continue", move |s: &mut Cursive| {
                    let model = get_current_mut_model(s);
                    model.temp.up_migration_written = false;
                    model.temp.down_migration_written = false;

                    let res = futures::executor::block_on(model.delete_migration(idx as i64));

                    match res {
                        Ok(_) => {}
                        Err(e) => {
                            s.add_layer(Dialog::info(e));
                            return;
                        }
                    };

                    remove_select_item(s, "migration_list", idx);

                    s.pop_layer();
                    s.pop_layer();
                }),
        );
    };

    let on_cancel = |s: &mut Cursive| {
        let model = get_current_mut_model(s);
        model.temp.up_migration_written = false;
        model.temp.down_migration_written = false;

        s.pop_layer();
    };

    s.add_layer(
        Dialog::new()
            .title(migration.name)
            .content(list.scrollable())
            .padding_lrtb(1, 1, 1, 0)
            .button("submit", on_submit)
            .button("delete", on_delete)
            .button("cancel", on_cancel),
    );
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
