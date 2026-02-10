use dirs;
use rbrain::ui::cli::app;

fn main() {
    let data_dir = dirs::home_dir()
        .expect("Nie znaleziono katalogu domowego")
        .join(".local/share/rbrain");

    std::fs::create_dir_all(&data_dir).expect("Nie udało się utworzyć katalogu");

    let db_path = data_dir.join("rbrain.db");
    let mut app = app::App::new(db_path.to_str().unwrap());
    app.run();
}
