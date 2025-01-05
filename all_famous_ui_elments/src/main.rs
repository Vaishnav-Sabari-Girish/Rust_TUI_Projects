use cursive::views::*;
use cursive::Cursive;

fn main() {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    let elements = LinearLayout::vertical()
        .child(TextView::new("This is TextView"))
        .child(Button::new("This is a Button", open_dialog_box))
        .child(Button::new("Click me for more - Part 1", part1));

    siv.add_layer(elements);
    siv.run();
}

fn callback(s: &mut Cursive) {
    let text = TextView::new("You did something");
    s.add_layer(text);
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

fn part1(s: &mut Cursive) {}
