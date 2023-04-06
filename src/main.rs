use devy::{AppConfig, build_new_app};

mod run;
fn main() {
    let app_config = build_new_app().unwrap();

    let command = app_config.command.as_str();

    let result = match command {
        "base64" => run::process_base64(app_config),
        _ => unreachable!("Cannoy determine command")
    };

    if result.is_ok() {
        println!("{}", result.unwrap())
    } else {
        panic!("{:?}", result.unwrap())
    }
}