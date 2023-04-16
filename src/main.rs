use base64::{engine::general_purpose, Engine as _};
use reqwest::header::{AUTHORIZATION, USER_AGENT};

use reqwest::blocking::Client;
use std::env;
use std::error::Error;

// determine the package name and version at compile time usiong the env! macro
static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " ",
    "ianmichaelkenney@gmail.com"
);
// define the environment variables used
static USERNAMEVAR: &str = "USERNAMENOIP";
static PASSWORDVAR: &str = "PASSWORDNOIP";
static HOSTNAMEVAR: &str = "HOSTNAMENOIP";

fn update_noip(client: Client, new_ip: Option<String>) -> Result<(), Box<dyn Error>> {
    // check for username in env
    let username = match env::var(USERNAMEVAR) {
        Ok(v) => v,
        Err(_) => {
            panic!("Missing username. Set {:}", USERNAMEVAR);
        }
    };
    // check for password in env
    let password = match env::var(PASSWORDVAR) {
        Ok(v) => v,
        Err(_) => panic!("Missing password. Set {:}", PASSWORDVAR),
    };

    let hostname = match env::var(HOSTNAMEVAR) {
        Ok(v) => v,
        Err(_) => panic!("Missing hostname. Set {:}", HOSTNAMEVAR),
    };

    let data = username + ":" + &password;
    println!("Encoding: {:}", data);

    let encoded = general_purpose::STANDARD.encode(&data);
    println!("{:}", data);
    let req_url = String::from("https://dynupdate.no-ip.com/nic/update?hostname=") + &hostname;

    let request = client
        .get(req_url)
        .header(USER_AGENT, APP_USER_AGENT)
        .header(AUTHORIZATION, encoded);

    // let resp = request.send()?.text()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::builder().user_agent(APP_USER_AGENT).build()?;
    update_noip(client.clone(), None)?;
    Ok(())
}
