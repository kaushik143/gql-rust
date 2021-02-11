#[derive(Clone)]
pub struct Config {
    base_url: String,
    client: actix_web::client::Client,
}

// fn get_var<F: FromStr>(key: &str) -> Result<F, String> {
//     match env::var(key).map(|val| val.parse::<F>()) {
//         Ok(Ok(val)) => Ok(val),
//         Ok(Err(_)) => Err(format!("invalid env var: {}", key)),
//         Err(_) => Err(format!("missing env var: {}", key)),
//     }
// }

pub fn load_config() -> Result<Config, String> {
    Ok(Config {
        base_url: "http:://api.dream11.com".to_owned(),
        client: actix_web::client::Client::new(),
    })
}
