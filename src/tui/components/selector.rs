use cursive::{view::Nameable, views::SelectView, Cursive, View};

pub fn select_component<F>(items: Vec<String>, refname: &str, cb: F) -> impl View
where
    F: Fn(&mut Cursive, &usize) + 'static,
{
    let mut selectview = SelectView::new();

    for i in 0..items.len() {
        selectview.add_item(items.get(i).unwrap(), i);
    }

    selectview.on_submit(cb).with_name(refname)
}

pub fn add_to_select_component(s: &mut Cursive, refname: &str, item: String, idx: usize) {
    let mut selectview = s.find_name::<SelectView<usize>>(refname).unwrap();
    selectview.add_item(item, idx);
}

pub fn update_select_component(s: &mut Cursive, refname: &str, items: Vec<String>) {
    let mut selectview = s.find_name::<SelectView<usize>>(refname).unwrap();

    selectview.clear();
    for i in 0..items.len() {
        selectview.add_item(items.get(i).unwrap(), i);
    }
}

pub fn select_component_with_ids<F>(
    items: Vec<String>,
    ids: Vec<usize>,
    refname: &str,
    cb: F,
) -> impl View
where
    F: Fn(&mut Cursive, &usize) + 'static,
{
    let mut selectview = SelectView::new();

    for i in 0..items.len() {
        selectview.add_item(items.get(i).unwrap(), ids.get(i).unwrap().to_owned());
    }

    selectview.on_submit(cb).with_name(refname)
}

pub fn update_select_component_with_ids(
    s: &mut Cursive,
    refname: &str,
    items: Vec<String>,
    ids: Vec<usize>,
) {
    let mut selectview = s.find_name::<SelectView<usize>>(refname).unwrap();

    selectview.clear();
    for i in 0..items.len() {
        selectview.add_item(items.get(i).unwrap(), ids.get(i).unwrap().to_owned());
    }
}
