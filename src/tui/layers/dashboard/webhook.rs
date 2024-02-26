use cursive::{
    view::{Nameable, Resizable, Scrollable},
    views::{Button, Dialog, EditView, ListView, NamedView, SelectView, TextArea, TextView},
    Cursive,
};

use crate::{
    queries::model::Webhook,
    tui::{
        components::{
            self,
            selector::{add_select_item, remove_select_item, select_component, update_select_item},
        },
        model::Sidebar,
        utils::{get_current_model, get_current_mut_model, get_data_from_refname},
    },
};

pub fn webhook_dashboard(s: &mut Cursive) -> NamedView<Dialog> {
    let model = get_current_mut_model(s);

    let on_select = |s: &mut Cursive, idx: &usize| {
        edit_webhook(s, *idx);
    };

    let optional_webhooks = futures::executor::block_on(model.get_all_webhooks());

    let mut webhooks = vec![];

    match optional_webhooks {
        Ok(w) => {
            webhooks = w;
        }
        Err(e) => s.add_layer(Dialog::info(e)),
    }
    let webhook_list = components::selector::select_component(
        webhooks
            .into_iter()
            .map(|r| (r.id as usize, r.name))
            .collect(),
        "webhook_list",
        on_select,
    );

    Dialog::new()
        .title("Webhook")
        .content(webhook_list)
        .padding_lrtb(1, 1, 1, 0)
        .button("Add Webhook", add_webhook)
        .with_name(Sidebar::Webhook.to_string())
}

fn add_webhook(s: &mut Cursive) {
    let on_submit = |s: &mut Cursive| {
        let webhook_ref = get_data_from_refname::<EditView>(s, "add_webhook_name");
        let webhook_name = webhook_ref.get_content().to_string();

        let model = get_current_mut_model(s);
        let res = futures::executor::block_on(model.add_new_webhook(webhook_name.clone()));

        match res {
            Ok(i) => {
                add_select_item(s, "webhook_list", webhook_name, i as usize);

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
            .title("Add Webhook Name")
            .padding_lrtb(1, 1, 1, 0)
            .content(textedit.with_name("add_webhook_name"))
            .button("submit", on_submit)
            .button("cancel", on_cancel),
    );
}

fn edit_webhook(s: &mut Cursive, idx: usize) {
    let model = get_current_model(s);

    let optional_webhook = futures::executor::block_on(model.get_webhook_by_id(idx as i64));
    let webhook = match optional_webhook {
        Ok(w) => w,
        Err(e) => {
            s.add_layer(Dialog::info(e));
            return;
        }
    };

    let mut list = ListView::new();
    list.add_child(
        "Label",
        EditView::new()
            .content(webhook.name)
            .with_name("edit_webhook_label"),
    );

    list.add_child(
        "Type",
        Button::new(webhook.exec_type, |s: &mut Cursive| {
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
                        get_data_from_refname::<Button>(s, "edit_webhook_type");

                    let (_, selected_label) = items.get(*idx).unwrap();

                    button_label_ref.set_label(selected_label);

                    s.pop_layer();
                },
            );

            s.add_layer(Dialog::new().content(exec_types.scrollable()).button(
                "cancel",
                |s: &mut Cursive| {
                    s.pop_layer();
                },
            ));

            let button_label_ref = get_data_from_refname::<Button>(s, "edit_webhook_type");
            let btn_label = button_label_ref.label().replace(['<', '>'], "").to_string();

            let items: Vec<(usize, String)> = vec![
                (0, "get".to_string()),
                (1, "post".to_string()),
                (2, "put".to_string()),
                (3, "delete".to_string()),
            ];

            let optional_i = items.iter().find(|(_, f)| *f == btn_label);
            match optional_i {
                Some((i, _)) => {
                    let mut exec_type_ref =
                        get_data_from_refname::<SelectView<usize>>(s, "exec_type");
                    exec_type_ref.set_selection(*i);
                }
                None => todo!(),
            }
        })
        .with_name("edit_webhook_type"),
    );

    list.add_child(
        "Url",
        EditView::new()
            .content(webhook.url)
            .with_name("edit_webhook_url"),
    );

    list.add_child(
        "Args",
        TextArea::new()
            .content(webhook.args.to_string())
            .with_name("edit_webhook_args")
            .fixed_height(5),
    );

    let on_submit = move |s: &mut Cursive| {
        let label_ref = get_data_from_refname::<EditView>(s, "edit_webhook_label");
        let label = label_ref.get_content().to_string();

        let exec_type_ref = get_data_from_refname::<Button>(s, "edit_webhook_type");
        let exec_type = exec_type_ref.label().replace(['<', '>'], "").to_string();

        let url_ref = get_data_from_refname::<EditView>(s, "edit_webhook_url");
        let url = url_ref.get_content().to_string();

        let args_ref = get_data_from_refname::<TextArea>(s, "edit_webhook_args");
        let args = args_ref.get_content().to_string();

        let model = get_current_mut_model(s);

        let res = futures::executor::block_on(model.edit_webhook(Webhook {
            id: idx as i64,
            name: label.clone(),
            exec_type: exec_type.clone(),
            url,
            args,
        }));

        if let Err(e) = res {
            s.add_layer(Dialog::info(e));
            return;
        }

        update_select_item(s, "webhook_list", label.clone(), idx);

        s.pop_layer();
    };

    let on_delete = move |s: &mut Cursive| {
        s.add_layer(
            Dialog::new()
                .content(TextView::new("Are you sure you want to remove webhook?"))
                .button("cancel", |s: &mut Cursive| {
                    s.pop_layer();
                })
                .button("continue", move |s: &mut Cursive| {
                    let model = get_current_mut_model(s);

                    let res = futures::executor::block_on(model.delete_webhook(idx as i64));
                    match res {
                        Ok(_) => {}
                        Err(e) => {
                            s.add_layer(Dialog::info(e));
                            return;
                        }
                    };

                    remove_select_item(s, "webhook_list", idx);

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
            .content(list.scrollable())
            .padding_lrtb(1, 1, 1, 0)
            .button("submit", on_submit)
            .button("delete", on_delete)
            .button("cancel", on_cancel),
    );
}
