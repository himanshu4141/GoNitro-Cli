use rocket::config::{Config, Environment, LoggingLevel};
use rocket::http::RawStr;
use rocket::State;
use std::sync::{mpsc, Mutex};

#[derive(Debug)]
pub struct AuthInfo {
  pub code: String,
}

impl AuthInfo {
  pub fn new(code: &RawStr) -> Self {
    Self {
      code: String::from(code.as_str()),
    }
  }
}

pub type AuthResult = Result<AuthInfo, String>;
pub type Transmitter = mpsc::Sender<AuthResult>;
pub type TxMutex<'req> = State<'req, Mutex<Transmitter>>;

// -- Routes

#[get("/?<code>")]
fn success(code: &RawStr, tx_mutex: TxMutex) -> &'static str {
  let tx = tx_mutex.lock().unwrap();
  tx.send(Ok(AuthInfo::new(code))).unwrap();
  "✅ You may close this browser tab and return to the terminal."
}

#[get("/?<error>", rank = 2)]
fn error(error: &RawStr, tx_mutex: TxMutex) -> String {
  let tx = tx_mutex.lock().unwrap();
  tx.send(Err(String::from(error.as_str()))).unwrap();
  format!("Error: {}, please return to the terminal.", error)
}

// -- Server 

pub fn start(tx: Transmitter) {
  let config = Config::build(Environment::Development)
    .port(9001)
    .log_level(LoggingLevel::Off)
    .workers(1)
    .finalize()
    .unwrap();
  rocket::custom(config)
    .mount("/callback", routes![success, error])
    .manage(Mutex::new(tx))
    .launch();
}