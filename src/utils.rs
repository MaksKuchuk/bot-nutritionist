use teloxide::types::{KeyboardButton, KeyboardMarkup};

pub fn create_keyboard(items_in_row: usize, items: Vec<&str>) -> KeyboardMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];

    for i in items.chunks(items_in_row) {
        let row = i
            .iter()
            .map(|&i| KeyboardButton::new(i.to_owned()))
            .collect();

        keyboard.push(row);
    }

    KeyboardMarkup::new(keyboard)
        .resize_keyboard(true)
        .one_time_keyboard(true)
        .persistent()
}
