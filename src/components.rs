use super::*;

pub fn build_search() -> gtk::Box {
    let stack = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let grid = build_grid(all_emojis());

    let searchbox = SearchEntry::builder().build();

    stack.append(&searchbox);
    stack.append(&grid);

    searchbox.connect_search_changed(move |sb| {
        let parent: gtk::Box = unsafe { sb.parent().unwrap().unsafe_cast() };
        parent.remove(&parent.last_child().unwrap());

        parent.append(&build_grid(all_emojis().filter(|e| {
            e.shortcodes().any(|sc| sc.contains(&sb.text().to_string()))
        })));
    });

    stack
}

pub fn build_grid(
    emojis: impl Iterator<Item = &'static Emoji>,
) -> ScrolledWindow {
    let grid = Grid::builder()
        .column_spacing(10)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .row_homogeneous(true)
        .column_homogeneous(true)
        .build();

    let mut row = 0;
    let mut col = 0;

    for emoji in emojis {
        let button = Button::builder().label(emoji.to_string()).build();

        button.connect_clicked(on_emoji_picked);
        grid.attach(&button, col, row, 1, 1);

        col += 1;

        if col == EMOJIS_PER_ROW {
            col = 0;
            row += 1;
        }
    }

    ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .width_request(500)
        .height_request(400)
        .child(&grid)
        .build()
}
