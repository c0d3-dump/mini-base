use cursive::{views::ViewRef, Cursive, View};

use super::{components, jsondb, model::Model};

pub fn get_current_model(s: &mut Cursive) -> Model {
    s.with_user_data(|data: &mut Model| data.clone()).unwrap()
}

pub fn get_current_mut_model(s: &mut Cursive) -> &mut Model {
    s.user_data().unwrap()
}

pub fn get_data_from_refname<T>(s: &mut Cursive, refname: &str) -> ViewRef<T>
where
    T: View,
{
    s.find_name::<T>(refname).unwrap()
}

pub fn update_role_with_model(s: &mut Cursive) {
    let model = get_current_model(s);
    jsondb::to_json(model.clone());

    let role_list_items = model
        .rolelist
        .into_iter()
        .map(|r| r)
        .collect::<Vec<String>>();

    components::selector::update_select_component(s, "role_list", role_list_items);
}

pub fn update_query_with_model(s: &mut Cursive) {
    let model = get_current_model(s);
    jsondb::to_json(model.clone());

    let query_list_items = model
        .querylist
        .into_iter()
        .map(|r| r.label)
        .collect::<Vec<String>>();

    components::selector::update_select_component(s, "query_list", query_list_items.clone());
    components::selector::update_select_component(s, "query_editor_list", query_list_items);
}
