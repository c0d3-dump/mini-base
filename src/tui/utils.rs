use cursive::{views::ViewRef, Cursive, View};

use crate::queries::Model;

pub fn get_current_model(s: &mut Cursive) -> Model {
    s.with_user_data(|data: &mut Model| data.clone()).unwrap()
}

pub fn get_current_mut_model(s: &mut Cursive) -> &mut Model {
    s.user_data().unwrap()
}

pub fn get_data_from_refname<T>(s: &mut Cursive, refname: &str) -> ViewRef<T>
where
    T: View,
{
    s.find_name::<T>(refname).unwrap()
}
