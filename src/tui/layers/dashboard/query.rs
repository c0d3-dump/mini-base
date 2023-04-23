use cursive::{
    direction::Orientation,
    view::{Nameable, Resizable},
    views::{Dialog, EditView, LinearLayout, ListView, NamedView, RadioGroup, ResizedView},
    Cursive, With,
};

use crate::tui::{
    components,
    model::{ExecType, Model, QueryList, Sidebar},
    utils::{
        get_current_model, get_current_mut_model, get_data_from_refname, update_query_with_model,
    },
};

pub fn query_dashboard(s: &mut Cursive) -> NamedView<ResizedView<Dialog>> {
    let model = get_current_model(s);

    let query_list_items = model
        .querylist
        .into_iter()
        .map(|r| r.label)
        .collect::<Vec<String>>();

    let on_select = |s: &mut Cursive, idx: &usize| {
        edit_query(s, *idx);
    };

    let query_list =
        components::selector::select_component(query_list_items, "query_list", on_select);

    let on_add_query = add_query;

    Dialog::new()
        .title("query")
        .content(query_list)
        .button("Add Query", on_add_query)
        .full_screen()
        .with_name(Sidebar::QUERY.to_string())
}

fn edit_query(s: &mut Cursive, idx: usize) {
    let query = get_current_model(s).querylist.get(idx).unwrap().to_owned();

    let mut list = ListView::new();
    list.add_child(
        "label",
        EditView::new()
            .content(&query.label)
            .with_name("edit_query_label"),
    );

    let mut boolean_group: RadioGroup<ExecType> = RadioGroup::new();
    list.add_child(
        "approval required",
        LinearLayout::new(Orientation::Horizontal)
            .child(boolean_group.button(ExecType::QUERY, "Query"))
            .child(
                boolean_group
                    .button(ExecType::EXECUTION, "Execution")
                    .with_if(query.exec_type == ExecType::EXECUTION, |b| {
                        b.select();
                    }),
            ),
    );

    let model = get_current_model(s);
    let all_rolelist = model.rolelist;

    let mut role_list = vec![];
    for ra in all_rolelist {
        if query.clone().role_access.contains(&ra) {
            role_list.push((ra.to_string(), true));
        } else {
            role_list.push((ra.to_string(), false));
        }
    }

    let check_box = components::checkbox_group::checkbox_group_component("role_list", role_list);
    list.add_child("role list", check_box);

    let on_submit = move |s: &mut Cursive| {
        let model = get_current_model(s);
        let all_rolelist = model.rolelist;

        let rolelist = components::checkbox_group::get_checked_data(s, all_rolelist);

        let label = s
            .call_on_name("edit_query_label", |view: &mut EditView| view.get_content())
            .unwrap()
            .to_string();

        let query = get_current_model(s).querylist.get(idx).unwrap().to_owned();
        let querylist = QueryList {
            label,
            exec_type: boolean_group.selection().as_ref().to_owned(),
            role_access: rolelist,
            query: query.query,
        };

        let model = get_current_mut_model(s);
        model.querylist[idx] = querylist;

        update_query_with_model(s);

        s.pop_layer();
    };

    let on_delete = move |s: &mut Cursive| {
        let model = get_current_mut_model(s);
        model.querylist.remove(idx);

        update_query_with_model(s);

        s.pop_layer();
    };

    let on_cancel = |s: &mut Cursive| {
        s.pop_layer();
    };

    s.add_layer(
        Dialog::new()
            .title("edit role")
            .content(list)
            .button("submit", on_submit)
            .button("delete", on_delete)
            .button("cancel", on_cancel),
    );
}

fn add_query(s: &mut Cursive) {
    let on_submit = |s: &mut Cursive| {
        let data = get_data_from_refname::<EditView>(s, "add_query_text");

        s.with_user_data(|m: &mut Model| {
            m.querylist.push(QueryList {
                label: data.get_content().to_string(),
                role_access: vec![],
                exec_type: ExecType::QUERY,
                query: "".to_string(),
            });
        })
        .unwrap();

        update_query_with_model(s);

        s.pop_layer();
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
