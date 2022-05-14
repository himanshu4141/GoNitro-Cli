// Required for Rocket code generation to work
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

use std::sync::mpsc;
use webbrowser;

mod server;
mod nitro;

fn main() {
    println!("
    ███╗░░██╗██╗████████╗██████╗░░█████╗░
    ████╗░██║██║╚══██╔══╝██╔══██╗██╔══██╗
    ██╔██╗██║██║░░░██║░░░██████╔╝██║░░██║
    ██║╚████║██║░░░██║░░░██╔══██╗██║░░██║
    ██║░╚███║██║░░░██║░░░██║░░██║╚█████╔╝
    ╚═╝░░╚══╝╚═╝░░░╚═╝░░░╚═╝░░╚═╝░╚════╝░
    ");
    println!("An un-official CLI to manage your Nitro Account.");
    println!("Lets first authenticate you.");

    let auth_url = nitro::auth_url();
    if webbrowser::open(&auth_url).is_err() {
      // Try manually
      println!("Visit the following URL to authorize your app with Nitro:");
      println!("{}\n", auth_url);
    }

    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
      server::start(tx);
    });

    match rx.recv().unwrap() {
        Ok(auth_info) => match nitro::exchange_token(&auth_info.code) {
          Ok(login) => {
            println!("{:#?}", login);
          }
          Err(error) => eprintln!("Error: {:#?}", error),
        },
        Err(error) => {
          eprintln!("Error: {:#?}", error);
          // Let the async server send its response
          // before the main thread exits.
          std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
