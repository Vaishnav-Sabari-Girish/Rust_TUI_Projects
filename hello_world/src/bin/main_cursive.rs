use cursive::views::TextView;

fn main() {
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(TextView::new("Hello World from Cursive. Press <q> to quit"));

    siv.run();
}
