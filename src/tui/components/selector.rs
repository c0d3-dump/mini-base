use cursive::{
    view::Nameable,
    views::{Dialog, SelectView},
    Cursive, View,
};

pub fn select_component<F>(s: &mut Cursive, items: Vec<String>, cb: F) -> impl View
where
    F: Fn(&mut Cursive, &usize) + 'static,
{
    let mut selectview = SelectView::new();

    for i in 0..items.len() {
        selectview.add_item(items.get(i).unwrap(), i);
    }

    selectview.on_submit(cb)
}
