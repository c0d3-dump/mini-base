use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, TextArea},
    Cursive, View,
};

pub fn editor_componant<F>(refname: &str, title: &str, cb: F, content: String) -> impl View
where
    F: Fn(&mut Cursive) + 'static,
{
    let textarea = TextArea::new();

    Dialog::new()
        .title(title)
        .padding_lrtb(1, 1, 1, 0)
        .content(textarea.content(content).with_name(refname).full_screen())
        .button("submit", cb)
        .button("cancel", |s| {
            s.pop_layer();
        })
}
