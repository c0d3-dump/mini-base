use cursive::{
    view::Nameable,
    views::{Dialog, ListView, SelectView},
    Cursive, View,
};

pub fn list_component<F, V>(items: Vec<(String, V)>, cb: F) -> impl View
where
    F: Fn(&mut Cursive, &String) + 'static,
    V: View + 'static,
{
    let mut list = ListView::new();

    for (label, view) in items {
        list.add_child(label, view);
    }

    list.on_select(cb)
}
