use std::ops::{Add, RangeInclusive, Sub};

use cosmic::{
    widget::{Button, Column, Row, Text, TextInput},
    Element,
};

use crate::ModifNodeMsg;

pub trait MyFrom<T> {
    fn from(value: T) -> Self;
}

impl MyFrom<i32> for u8 {
    fn from(value: i32) -> Self {
        value as u8
    }
}

impl MyFrom<&str> for Option<u8> {
    fn from(value: &str) -> Self {
        match value.parse::<u8>() {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }
}

pub fn input_line<'a, V, F>(
    info: &'a str,
    value: &'a V,
    cached_value: &str,
    unit: &'a str,
    range: &'a RangeInclusive<V>,
    map_value: F,
) -> Element<'a, ModifNodeMsg>
where
    V: Add<V, Output = V>,
    V: Sub<V, Output = V>,
    V: MyFrom<i32>,
    V: PartialOrd + Clone + ToString + PartialEq,
    Option<V>: for<'b> MyFrom<&'b str>,
    F: 'a + Fn(V, String) -> ModifNodeMsg,
{
    // `map_value` is moved in `on_input` so we procuce buttons messages before
    let plus_message = if range.end() > value {
        let new_value = value.clone() + MyFrom::from(1);
        let new_cached_value = new_value.to_string();
        Some(map_value(new_value, new_cached_value))
    } else {
        None
    };

    let sub_message = if range.start() < value {
        let new_value = value.clone() - MyFrom::from(1);
        let new_cached_value = new_value.to_string();
        Some(map_value(new_value, new_cached_value))
    } else {
        None
    };

    let mut input = TextInput::new("value", cached_value).on_input(move |s| {
        let final_value = match <Option<V> as MyFrom<_>>::from(&s) {
            Some(value_not_tested) => match range.contains(&value_not_tested) {
                true => value_not_tested,
                false => value.clone(),
            },
            None => value.clone(),
        };

        map_value(final_value, s)
    });

    let is_error = match <Option<V> as MyFrom<_>>::from(cached_value) {
        Some(value_from_string) => value != &value_from_string,
        None => true,
    };

    if is_error {
        input = input.error("this value is invalid");
    }

    Row::new()
        .push(Text::new(info))
        .push(
            Row::new()
                .push(Text::new(":"))
                .push(input)
                .push(Text::new(unit))
                .push(
                    Column::new()
                        .push(Button::new("+").on_press_maybe(plus_message))
                        .push(Button::new("-").on_press_maybe(sub_message)),
                ),
        )
        .into()
}
