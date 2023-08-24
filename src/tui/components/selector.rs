use cursive::{
    view::{Nameable, Scrollable},
    views::SelectView,
    Cursive, View,
};

pub fn select_component<F>(items: Vec<(usize, String)>, refname: &str, cb: F) -> impl View
where
    F: Fn(&mut Cursive, &usize) + 'static,
{
    let mut selectview = SelectView::new();

    for (idx, item) in items {
        selectview.add_item(item, idx);
    }

    selectview
        .on_submit(cb)
        .autojump()
        .with_name(refname)
        .scrollable()
}

pub fn update_select_item(
    s: &mut Cursive,
    refname: &str,
    item: String,
    idx: usize,
) -> Option<usize> {
    let mut selectview = s.find_name::<SelectView<usize>>(refname).unwrap();

    let binding = selectview
        .iter()
        .enumerate()
        .filter(|(_, (_, val))| *val == &idx)
        .map(|(i, (_, j))| (i, j.clone()))
        .collect::<Vec<(usize, usize)>>();

    let temp_idx = binding.first();

    match temp_idx {
        Some((i, j)) => {
            selectview.remove_item(*i);
            selectview.insert_item(*i, item, *j);

            selectview.set_selection(*i);
            Some(*i)
        }
        None => None,
    }
}

pub fn add_select_item(s: &mut Cursive, refname: &str, item: String, idx: usize) {
    let mut selectview = s.find_name::<SelectView<usize>>(refname).unwrap();

    selectview.add_item(item, idx);
}

pub fn remove_select_item(s: &mut Cursive, refname: &str, idx: usize) -> Option<usize> {
    let mut selectview = s.find_name::<SelectView<usize>>(refname).unwrap();

    let binding = selectview
        .iter()
        .enumerate()
        .filter(|(_, (_, val))| *val == &idx)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    let temp_idx = binding.first();

    match temp_idx {
        Some(i) => {
            selectview.remove_item(*i);
            Some(*i)
        }
        None => None,
    }
}
