use cursive::{
    view::{Nameable, Resizable, Scrollable},
    views::{Dialog, SelectView, TextArea},
    Cursive, View, XY,
};

use crate::tui::utils::{get_current_model, get_current_mut_model, get_data_from_refname};

pub fn editor_componant<F>(title: &str, cb: F, content: String) -> impl View
where
    F: Fn(&mut Cursive) + 'static + std::marker::Send + std::marker::Sync,
{
    let mut textarea = TextArea::new();
    textarea.set_on_edit(on_data_changes);

    Dialog::new()
        .title(title)
        .padding_lrtb(1, 1, 1, 0)
        .content(
            textarea
                .content(content)
                .with_name("base_editor")
                .full_screen(),
        )
        .button("submit", cb)
        .button("cancel", |s| {
            s.pop_layer();
        })
        .with_name("base_dialog")
}

fn on_data_changes(s: &mut Cursive, text: &str, cursor: usize) {
    let model = get_current_model(s);
    if model.temp.editor_popup_active {
        let mut v = get_data_from_refname::<Dialog>(s, "base_dialog");
        v.remove_popup_content();
    }

    let current_word = get_current_word(text, cursor);
    if let Some(word) = current_word {
        let suggestions = model.trie.starts_with(word);

        if suggestions.is_empty() {
            return;
        }

        let mut maxwidth = 0;
        for suggestion in &suggestions {
            let n = suggestion.len();
            maxwidth = if n > maxwidth { n } else { maxwidth };
        }

        let mut select_view = SelectView::new();

        select_view.add_all_str(suggestions);

        let mut v = get_data_from_refname::<Dialog>(s, "base_dialog");
        v.set_popup_content(
            select_view
                .on_submit(|s: &mut Cursive, i: &str| {
                    let model = get_current_mut_model(s);
                    model.temp.editor_popup_active = false;

                    let mut v = get_data_from_refname::<TextArea>(s, "base_editor");
                    let text = v.get_content();
                    let cursor = v.cursor();

                    let txt = replace_current_word(&text, cursor, i);
                    v.set_content(&txt);

                    let end = cursor
                        + txt[cursor..]
                            .find(char::is_whitespace)
                            .unwrap_or_else(|| txt.len() - cursor);

                    v.set_cursor(end);

                    let mut v = get_data_from_refname::<Dialog>(s, "base_dialog");
                    v.remove_popup_content();
                })
                .scrollable()
                .max_height(10)
                .max_width(maxwidth + 2),
            get_coordinates(text, cursor) + XY::new(0, 1),
        );
    }

    let model = get_current_mut_model(s);
    model.temp.editor_popup_active = true;
}

fn get_coordinates(text: &str, cursor: usize) -> XY<usize> {
    let mut x = 0;
    let mut y = 0;

    for (index, char) in text.chars().enumerate() {
        if index == cursor {
            break;
        }

        if char == '\n' {
            y += 1;
            x = 0;
        } else {
            x += 1;
        }
    }

    XY::new(x, y)
}

fn get_current_word(text: &str, cursor: usize) -> Option<&str> {
    let res = &text[0..cursor];
    if res.ends_with(' ') {
        return None;
    }

    let temp: Vec<&str> = res.split_whitespace().collect();

    temp.last().copied()
}

fn replace_current_word(text: &str, cursor: usize, word: &str) -> String {
    let start = text[..cursor]
        .rfind(char::is_whitespace)
        .map_or(0, |i| i + 1);

    let end = cursor
        + text[cursor..]
            .find(char::is_whitespace)
            .unwrap_or_else(|| text.len() - cursor);

    let mut res = text.to_string();
    res.replace_range(start..end, word);
    res
}

#[cfg(test)]
mod test {
    use super::get_current_word;

    #[test]
    fn test1() {
        let text = "se";
        let cursor = 2;
        let res = get_current_word(text, cursor);
        assert_eq!(res, Some("se"));
    }

    #[test]
    fn test2() {
        let text = "SELECT * fr";
        let cursor = 11;
        let res = get_current_word(text, cursor);
        assert_eq!(res, Some("fr"));
    }

    #[test]
    fn test3() {
        let text = "SELECT * from test";
        let cursor = 10;
        let res = get_current_word(text, cursor);
        assert_eq!(res, Some("f"));
    }
}
