use cursive::{
    views::{ListView}, View,
};

pub fn list_component<V>(items: Vec<(String, V)>) -> impl View
where
    V: View + 'static,
{
    let mut list = ListView::new();

    for (label, view) in items {
        list.add_child(label, view);
    }

    list
}
