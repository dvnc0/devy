use devy::build_new_app;

pub mod base64;
pub mod password;
pub mod api;

fn main() {
    let app_config = build_new_app().unwrap();

    let command = app_config.command.as_str();

    let result = match command {
        "base64" => base64::execute(app_config.sub_match).expect("There was an error processing your command"),
        "password" => password::execute(app_config.sub_match).expect("There was an error processing your command"),
        "api" => api::execute(app_config.sub_match).expect("some error"),
        _ => unreachable!("Cannot determine command")
    };

    println!("Result:\n{}", result)
}