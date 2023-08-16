use cursive::{
    view::Nameable,
    views::{Checkbox, ListView},
    Cursive, View, With,
};

pub fn checkbox_group_component(refname: &str, items: Vec<(String, bool)>) -> impl View {
    let mut list = ListView::new();

    for (label, is_checked) in items {
        let checkbox = Checkbox::new()
            .with_if(is_checked, |c| {
                c.check();
            })
            .with_name(&label);

        list.add_child(label, checkbox);
    }

    list.with_name(refname)
}

pub fn get_checked_data(s: &mut Cursive, all_items: Vec<String>) -> Vec<bool> {
    let mut items = vec![];

    for i in all_items {
        let checkbox = s.find_name::<Checkbox>(&i).unwrap();
        items.push(checkbox.is_checked());
    }

    items
}
