extern crate clap;
extern crate notify;
extern crate mysqlclient_sys;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate ignore;

//use std::sync::mpsc::channel;
//use std::time::Duration;
use clap::Arg;
//use notify::{Watcher, RecursiveMode, watcher};

static ME: &'static str = "Yoan Lecoq <yoanlecoq.io@gmail.com>";

mod early;
mod db;

use db::*;

fn main() {
    early::setup_env();
    early::setup_log();

    let matches = clap::App::new("Sync Files Via MySQL")
       .version("0.0.1")
       .author(ME)
       .about("Pretty self-explanatory")
       .arg(Arg::with_name("host")    .long("host")    .required(true).takes_value(true))
       .arg(Arg::with_name("port")    .long("port")    .required(true).takes_value(true))
       .arg(Arg::with_name("user")    .long("user")    .required(true).takes_value(true))
       .arg(Arg::with_name("database").long("database").required(true).takes_value(true))
       .arg(Arg::with_name("password").long("password").required(true).takes_value(true))
       .get_matches();

    let p = Credentials {
        port    : matches.value_of("port").unwrap().parse().unwrap(),
        host    : matches.value_of("host")    .unwrap().to_owned(),
        user    : matches.value_of("user")    .unwrap().to_owned(),
        database: matches.value_of("database").unwrap().to_owned(),
        password: matches.value_of("password").unwrap().to_owned(),
    };
    info!("Database backend info: {:#?}", Db::info());
    let _db = Db::new(&p).expect(&format!("Cound not connect to {}", PublicCredentials::from(p.clone())));

    for result in ignore::Walk::new("./") {
        match result {
            Ok(entry) => info!("{}", entry.path().display()),
            Err(err) => error!("ERROR: {}", err),
        }
    }

    /*
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_millis(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch("./", RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
           Ok(event) => println!("{:?}", event),
           Err(e) => println!("watch error: {:?}", e),
        }
    }
    */
}

