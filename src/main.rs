use rbrain::ui::cli::app;

fn main() {
    let mut app = app::App::new("rbrain_test.db");
    app.run();
}
