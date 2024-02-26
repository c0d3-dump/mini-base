use cursive::{
    align::Align,
    view::{Nameable, Scrollable},
    views::{Dialog, ListView, NamedView, TextView},
    Cursive,
};

use crate::tui::{model::Sidebar, utils::get_current_mut_model};

pub fn api_dashboard(s: &mut Cursive) -> NamedView<Dialog> {
    let apis = ListView::new();

    let apis = update_apis(s, apis);

    Dialog::new()
        .title("Api")
        .content(apis.with_name("server_apis").scrollable())
        .padding_lrtb(1, 1, 1, 0)
        .with_name(Sidebar::Api.to_string())
}

fn update_apis(s: &mut Cursive, mut apis: ListView) -> ListView {
    apis.add_child(
        "/auth/login",
        TextView::new("post").align(Align::center_right()),
    );
    apis.add_child(
        "/auth/signup",
        TextView::new("post").align(Align::center_right()),
    );
    apis.add_child(
        "/auth/logout",
        TextView::new("post").align(Align::center_right()),
    );
    apis.add_child(
        "/storage/upload",
        TextView::new("post").align(Align::center_right()),
    );
    apis.add_child(
        "/storage/delete",
        TextView::new("post").align(Align::center_right()),
    );
    apis.add_child(
        "/storage/generate-token",
        TextView::new("post").align(Align::center_right()),
    );

    let model = get_current_mut_model(s);
    let optional_queries = futures::executor::block_on(model.get_all_apis());

    match optional_queries {
        Ok(queries) => {
            for q in queries {
                apis.add_child(
                    format!("/api/{}", q.name),
                    TextView::new(q.exec_type).align(Align::center_right()),
                );
            }
        }
        Err(e) => {
            s.add_layer(Dialog::info(e));
        }
    }

    apis
}
