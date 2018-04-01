use std::env;
use env_logger;
use log::LevelFilter;

pub fn setup_env() {
    //env::set_var("RUST_LOG", "info");
    env::set_var("RUST_BACKTRACE", "full");
}

pub fn setup_log() {
    let mut builder = env_logger::Builder::new();
    builder.format(|buf, record| {
        use ::std::io::Write;
        let s = format!("{}", record.level());
        let s = s.chars().next().unwrap();
        writeln!(buf, "[{}] {}", s, record.args())
    }).filter(None, LevelFilter::Debug);

    if let Ok(rust_log) = env::var("RUST_LOG") {
        builder.parse(&rust_log);
    }
    builder.init();
}
