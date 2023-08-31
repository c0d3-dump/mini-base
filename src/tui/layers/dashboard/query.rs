use cursive::{
    align::Align,
    view::{Nameable, Resizable, Scrollable},
    views::{
        Button, Dialog, EditView, ListChild, ListView, NamedView, ResizedView, SelectView,
        TextArea, TextView,
    },
    Cursive,
};

use crate::{
    queries::model::{Query, QueryAccess},
    tui::{
        components::{
            self,
            checkbox_group::get_checked_data,
            selector::{add_select_item, remove_select_item, select_component, update_select_item},
        },
        model::Sidebar,
        utils::{get_current_model, get_current_mut_model, get_data_from_refname},
    },
};

pub fn query_dashboard(s: &mut Cursive) -> NamedView<ResizedView<Dialog>> {
    let model = get_current_mut_model(s);

    let on_select = |s: &mut Cursive, idx: &usize| {
        edit_query(s, *idx);
    };

    let optional_queries = futures::executor::block_on(model.get_all_queries());

    let mut queries = vec![];

    match optional_queries {
        Ok(q) => {
            queries = q;
        }
        Err(e) => s.add_layer(Dialog::info(e)),
    }

    let query_list = components::selector::select_component(
        queries
            .into_iter()
            .map(|r| (r.id as usize, r.name))
            .collect(),
        "query_list",
        on_select,
    );

    Dialog::new()
        .title("Query")
        .content(query_list)
        .padding_lrtb(1, 1, 1, 0)
        .button("Add Query", add_query)
        .full_screen()
        .with_name(Sidebar::QUERY.to_string())
}

fn edit_query(s: &mut Cursive, idx: usize) {
    let model = get_current_model(s);

    let optional_query = futures::executor::block_on(model.get_query_by_id(idx as i64));
    let query;

    match optional_query {
        Ok(q) => {
            query = q;
        }
        Err(e) => {
            s.add_layer(Dialog::info(e));
            return;
        }
    }

    let mut list = ListView::new();
    list.add_child(
        "Label",
        EditView::new()
            .content(query.name)
            .with_name("edit_query_label"),
    );

    list.add_child(
        "Type",
        Button::new(query.exec_type, |s: &mut Cursive| {
            let items: Vec<(usize, String)> = vec![
                (0, "get".to_string()),
                (1, "post".to_string()),
                (2, "put".to_string()),
                (3, "delete".to_string()),
            ];

            let exec_types = select_component(
                items.clone(),
                "exec_type",
                move |s: &mut Cursive, idx: &usize| {
                    let mut button_label_ref =
                        get_data_from_refname::<Button>(s, "exec_type_label");

                    let (_, selected_label) = items.get(*idx).unwrap();

                    button_label_ref.set_label(selected_label);

                    s.pop_layer();
                },
            );

            s.add_layer(exec_types);

            let button_label_ref = get_data_from_refname::<Button>(s, "exec_type_label");
            let btn_label = button_label_ref
                .label()
                .replace("<", "")
                .replace(">", "")
                .to_string();

            let items: Vec<(usize, String)> = vec![
                (0, "get".to_string()),
                (1, "post".to_string()),
                (2, "put".to_string()),
                (3, "delete".to_string()),
            ];

            let optional_i = items.iter().find(|(_, f)| f.to_string() == btn_label);
            match optional_i {
                Some((i, _)) => {
                    let mut exec_type_ref =
                        get_data_from_refname::<SelectView<usize>>(s, "exec_type");
                    exec_type_ref.set_selection(*i);
                }
                None => todo!(),
            }
        })
        .with_name("exec_type_label"),
    );

    list.add_child(
        "Access",
        Button::new("", move |s: &mut Cursive| {
            let model = get_current_mut_model(s);

            if !model.temp.query_access_update {
                let optional_roles =
                    futures::executor::block_on(model.get_query_access_by_id(idx as i64));

                match optional_roles {
                    Ok(r) => {
                        model.temp.query_access = r;
                        model.temp.query_access_update = true;
                    }
                    Err(e) => {
                        s.add_layer(Dialog::info(e));
                        return;
                    }
                }
            }

            let check_box = components::checkbox_group::checkbox_group_component(
                "role_list",
                model
                    .temp
                    .query_access
                    .iter()
                    .map(|role| (role.name.clone(), role.has_access))
                    .collect(),
            );

            let on_submit = |s: &mut Cursive| {
                let model = get_current_mut_model(s);

                let items = model
                    .temp
                    .query_access
                    .iter()
                    .map(|r| r.name.clone())
                    .collect();

                let temp = get_checked_data(s, items);

                let model = get_current_mut_model(s);

                model.temp.query_access = model
                    .temp
                    .query_access
                    .iter()
                    .enumerate()
                    .map(|(i, r)| QueryAccess {
                        id: r.id,
                        name: r.name.clone(),
                        has_access: *temp.get(i).unwrap(),
                    })
                    .collect();

                s.pop_layer();
            };

            s.add_layer(
                Dialog::new()
                    .content(check_box.scrollable())
                    .button("submit", on_submit)
                    .button("cancel", |s: &mut Cursive| {
                        s.pop_layer();
                    }),
            );
        }),
    );

    list.add_child(
        "Editor",
        Button::new("", move |s: &mut Cursive| {
            let model = get_current_mut_model(s);

            let mut query_string: String = model.temp.query_string.clone();

            if !model.temp.query_written {
                let optional_query_string =
                    futures::executor::block_on(model.get_query_string_by_id(idx as i64));

                match optional_query_string {
                    Ok(s) => {
                        query_string = s.query;
                        model.temp.query_written = true;
                        model.temp.query_string = query_string.clone();
                    }
                    Err(e) => {
                        s.add_layer(Dialog::info(e));
                        return;
                    }
                }
            }

            let on_submit = move |s: &mut Cursive| {
                let query_ref = get_data_from_refname::<TextArea>(s, "query_editor");

                let model = get_current_mut_model(s);
                model.temp.query_string = query_ref.get_content().to_string();

                s.pop_layer();
            };

            s.add_layer(components::editor::editor_componant(
                "query_editor",
                "Editor",
                on_submit,
                query_string,
            ));
        }),
    );

    let on_submit = move |s: &mut Cursive| {
        let label_ref = get_data_from_refname::<EditView>(s, "edit_query_label");
        let label = label_ref.get_content().to_string();

        let mut exec_type: String = "get".to_string();
        s.call_on_name("exec_type_label", |v: &mut Button| {
            exec_type = v.label().replace("<", "").replace(">", "").to_string();
        });

        let model = get_current_mut_model(s);

        let res1 = futures::executor::block_on(model.edit_query(Query {
            id: idx as i64,
            name: label.clone(),
            exec_type: exec_type.clone(),
        }));

        match res1 {
            Ok(_) => {}
            Err(e) => {
                s.add_layer(Dialog::info(e));
                return;
            }
        }

        let query_access = model.temp.query_access.clone();
        model.temp.query_access.clear();

        if model.temp.query_access_update {
            model.temp.query_access_update = false;

            let res2: Result<u64, String> =
                futures::executor::block_on(model.edit_query_access(idx as i64, query_access));

            match res2 {
                Ok(_) => {}
                Err(e) => {
                    s.add_layer(Dialog::info(e));
                    return;
                }
            }
        }

        if model.temp.query_written {
            model.temp.query_written = false;

            let query_string = model.temp.query_string.clone();
            model.temp.query_string.clear();

            let res3: Result<u64, String> =
                futures::executor::block_on(model.edit_query_string(idx as i64, query_string));

            match res3 {
                Ok(_) => {}
                Err(e) => {
                    s.add_layer(Dialog::info(e));
                    return;
                }
            }
        }

        let res = update_select_item(s, "query_list", label.clone(), idx);
        match res {
            Some(i) => {
                s.call_on_name("server_apis", move |list: &mut ListView| {
                    let l = list.row_mut(i + 6);
                    match l {
                        ListChild::Row(_, _) => {
                            *l = ListChild::Row(
                                format!("/api/{}", label.clone()),
                                Box::new(TextView::new(exec_type).align(Align::center_right())),
                            );
                        }
                        _ => {}
                    }
                });
            }
            None => {}
        }

        s.pop_layer();
    };

    let on_delete = move |s: &mut Cursive| {
        s.add_layer(
            Dialog::new()
                .content(TextView::new(
                    "Are you sure you want to remove remove query?",
                ))
                .button("cancel", |s: &mut Cursive| {
                    s.pop_layer();
                })
                .button("continue", move |s: &mut Cursive| {
                    let model = get_current_mut_model(s);
                    model.temp.query_written = false;
                    model.temp.query_access_update = false;

                    let res = futures::executor::block_on(model.delete_query(idx as i64));

                    match res {
                        Ok(_) => {}
                        Err(e) => {
                            s.add_layer(Dialog::info(e));
                            return;
                        }
                    };

                    let res = remove_select_item(s, "query_list", idx);
                    match res {
                        Some(i) => {
                            s.call_on_name("server_apis", |list: &mut ListView| {
                                list.remove_child(i + 6);
                            });
                        }
                        None => {}
                    }

                    s.pop_layer();
                    s.pop_layer();
                }),
        );
    };

    let on_cancel = |s: &mut Cursive| {
        let model = get_current_mut_model(s);
        model.temp.query_written = false;
        model.temp.query_access_update = false;

        s.pop_layer();
    };

    s.add_layer(
        Dialog::new()
            .title("Edit Query")
            .content(list)
            .padding_lrtb(1, 1, 1, 0)
            .button("submit", on_submit)
            .button("delete", on_delete)
            .button("cancel", on_cancel),
    );
}

fn add_query(s: &mut Cursive) {
    let on_submit = |s: &mut Cursive| {
        let label_ref = get_data_from_refname::<EditView>(s, "add_query_text");
        let label_data = label_ref.get_content().to_string();

        let model = get_current_mut_model(s);

        let res = futures::executor::block_on(model.add_new_query(label_data.clone()));

        match res {
            Ok(i) => {
                add_select_item(s, "query_list", label_data.clone(), i as usize);

                s.call_on_name("server_apis", |list: &mut ListView| {
                    list.add_child(
                        format!("/api/{}", label_data),
                        TextView::new("get").align(Align::center_right()),
                    );
                });

                s.pop_layer();
            }
            Err(e) => {
                s.add_layer(Dialog::info(e));
                return;
            }
        };
    };

    let on_cancel = |s: &mut Cursive| {
        s.pop_layer();
    };

    let textedit = EditView::new();

    s.add_layer(
        Dialog::new()
            .title("add query")
            .content(textedit.with_name("add_query_text"))
            .padding_lrtb(1, 1, 1, 0)
            .button("submit", on_submit)
            .button("cancel", on_cancel),
    );
}
