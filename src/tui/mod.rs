use cursive::{
    view::Resizable,
    views::{Dialog, SelectView, TextArea},
};

mod style;

pub fn run() {
    let mut app = cursive::default();

    // app.set_theme(style::get_theme());

    app.add_layer(
        Dialog::new().title("mini base").content(
            SelectView::new()
                .item_str("0-18")
                .item_str("19-30")
                .item_str("19-30")
                .item_str("31-40")
                .item_str("41+")
                .on_submit(|s, item| {
                    let content = match item {
                        "0-18" => "Content number one",
                        "41+" => "Content number two! Much better!",
                        _ => unreachable!("no such item"),
                    };

                    s.pop_layer();
                    s.add_layer(TextArea::new().full_screen());
                }),
        ),
    );

    app.run();
}
