use devy::build_new_app;

pub mod base64;
fn main() {
    let app_config = build_new_app().unwrap();

    let command = app_config.command.as_str();

    let result = match command {
        "base64" => base64::execute(app_config.sub_match),
        _ => unreachable!("Cannot determine command")
    };

    if result.is_ok() {
        println!("{}", result.unwrap())
    } else {
        panic!("{:?}", result.unwrap())
    }
}