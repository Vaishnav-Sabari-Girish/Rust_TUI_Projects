use cursive::views::{Button, Dialog};
use cursive::Cursive;

fn main() {
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::text("Choose the currect button to quit")
            .title("Dialog Box")
            .button("Next", show_next),
    );

    siv.run();
}

fn show_next(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("Wanna quit ?")
            .title("Quit ?")
            .button("Yes", |s| s.quit())
            .button("No", |s| you_sure(s, "You Sure about that ? ")),
    );
}

fn you_sure(s: &mut Cursive, msg: &str) {
    s.pop_layer();
    s.add_layer(
        Dialog::text(msg)
            .title("You sure ? ")
            .button("Doesn't matter , click me or you're dead", |s| s.quit()),
    );
}
