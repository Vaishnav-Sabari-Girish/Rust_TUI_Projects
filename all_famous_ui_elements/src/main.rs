use cursive::align::*;
use cursive::traits::*;
use cursive::views::*;
use cursive::Cursive;

fn main() {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    let elements = LinearLayout::vertical()
        .child(TextView::new("This is TextView"))
        .child(Button::new("This is a Button", open_dialog_box))
        .child(Button::new("Click me for more - Part 1", part1))
        .child(Button::new("Click me for more - Part 2", part2))
        .child(Button::new("Exit", |s| s.quit()));

    siv.add_layer(elements);
    siv.run();
}

fn open_dialog_box(s: &mut Cursive) {
    fn back(s: &mut Cursive) {
        s.pop_layer();
    }
    let dialog = Dialog::new()
        .button("Back", back)
        .button("Quit", |s| s.quit())
        .title("Dialog Box");
    s.add_layer(dialog);
}

fn part1(s: &mut Cursive) {
    let dialog_elements = Dialog::around(
        LinearLayout::vertical()
            .child(TextView::new("Checkbox"))
            .child(Checkbox::new().checked().with_name("check"))
            .child(TextView::new("EditView"))
            .child(
                EditView::new()
                    .on_submit(callback)
                    .with_name("text")
                    .fixed_width(20),
            ),
    )
    .title("Part - 1 Widgets")
    .button("Back", back);
    s.add_layer(dialog_elements);
}

fn part2(s: &mut Cursive) {
    let dialog_elements = Dialog::around(
        LinearLayout::vertical()
            .child(TextView::new("TextArea"))
            .child(TextArea::new().content(""))
            .child(TextView::new("SelectView"))
            .child(TextView::new("").with_name("text"))
            .child(
                SelectView::new()
                    .h_align(HAlign::Center)
                    .item("Point 1", 1)
                    .item("Point 2", 2)
                    .item("Point 3", 3)
                    .on_submit(|s, item| {
                        let chosen = match *item {
                            1 => "Point 1",
                            2 => "Point 2",
                            3 => "Point 3",
                            _ => unreachable!("No such Point"),
                        };

                        s.add_layer(Dialog::info(format!("You chose \"{}\"", chosen)));
                    }),
            ),
    )
    .title("Part - 2 Widgets")
    .button("Back", back);

    s.add_layer(dialog_elements);
}

fn callback(s: &mut Cursive, text: &str) {
    s.add_layer(Dialog::info(format!("You typed \"{}\" in EditView", text)));
}

fn back(s: &mut Cursive) {
    s.pop_layer();
}
