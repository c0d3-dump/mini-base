use cursive::{view::Nameable, views::EditView, Cursive, View};

pub fn textedit_component<F>(refname: String, cb: F) -> impl View
where
    F: Fn(&mut Cursive, &str) + 'static,
{
    let edit = EditView::new();

    edit.on_submit(cb).with_name(refname)
}
