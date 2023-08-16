use cursive::{
    direction::Orientation,
    view::{Nameable, Resizable, Scrollable},
    views::{Button, Dialog, EditView, LinearLayout, ListView, NamedView, RadioGroup, ResizedView},
    Cursive, With,
};

use crate::{
    queries::model::{Query, QueryAccess},
    tui::{
        components::{
            self,
            checkbox_group::get_checked_data,
            selector::{add_select_item, remove_select_item, update_select_item},
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
        .title("query")
        .content(query_list)
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

    let mut boolean_group: RadioGroup<String> = RadioGroup::new();
    list.add_child(
        "Type",
        LinearLayout::new(Orientation::Horizontal)
            .child(boolean_group.button("fetch".to_string(), "Fetch"))
            .child(
                boolean_group
                    .button("execute".to_string(), "Execute")
                    .with_if(query.exec_type == "execute".to_string(), |b| {
                        b.select();
                    }),
            ),
    );

    let model = get_current_model(s);

    let optional_roles = futures::executor::block_on(model.get_query_access_by_id(idx as i64));

    match optional_roles {
        Ok(r) => {
            let model = get_current_mut_model(s);
            model.temp.query_access = r;
        }
        Err(e) => {
            s.add_layer(Dialog::info(e));
            return;
        }
    }

    list.add_child(
        "Access",
        Button::new("", |s: &mut Cursive| {
            let model = get_current_mut_model(s);

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

    let on_submit = move |s: &mut Cursive| {
        let label_ref = get_data_from_refname::<EditView>(s, "edit_query_label");

        let label = label_ref.get_content().to_string();

        let exec_type = boolean_group.selection().as_ref().to_owned();

        let mut model = get_current_model(s);

        let res1 = futures::executor::block_on(model.edit_query(Query {
            id: idx as i64,
            name: label.clone(),
            exec_type,
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

        let res2: Result<u64, String> =
            futures::executor::block_on(model.edit_query_access(idx as i64, query_access));

        match res2 {
            Ok(_) => {}
            Err(e) => {
                s.add_layer(Dialog::info(e));
                return;
            }
        }

        update_select_item(s, "query_list", label, idx);

        s.pop_layer();
    };

    let on_delete = move |s: &mut Cursive| {
        let model = get_current_mut_model(s);
        let res = futures::executor::block_on(model.delete_query(idx as i64));

        match res {
            Ok(_) => {}
            Err(e) => {
                s.add_layer(Dialog::info(e));
                return;
            }
        };

        remove_select_item(s, "query_list", idx);

        s.pop_layer();
    };

    let on_cancel = |s: &mut Cursive| {
        s.pop_layer();
    };

    s.add_layer(
        Dialog::new()
            .title("edit query")
            .content(list)
            .button("submit", on_submit)
            .button("delete", on_delete)
            .button("cancel", on_cancel),
    );
}

fn add_query(s: &mut Cursive) {
    let on_submit = |s: &mut Cursive| {
        let label_ref = get_data_from_refname::<EditView>(s, "add_query_text");

        let model = get_current_mut_model(s);

        let res =
            futures::executor::block_on(model.add_new_query(label_ref.get_content().to_string()));

        match res {
            Ok(i) => {
                add_select_item(
                    s,
                    "query_list",
                    label_ref.get_content().to_string(),
                    i as usize,
                );

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
            .button("submit", on_submit)
            .button("cancel", on_cancel),
    );
}
