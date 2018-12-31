extern crate log;

use std::env;
use log::*;
use log::{Record, Level, Metadata};
use log::{SetLoggerError};
use std::collections::HashMap;

struct TinyLogger {
    global:     Level,
    targets:    HashMap<String, Level>
}

impl log::Log for TinyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let mut lookup = String::new();
        for q in metadata.target().split("::") {
            lookup.push_str(q);
            lookup.push_str("::");
            if let Some(l) = self.targets.get(&lookup) {
                return metadata.level() <= *l;
            }
        }
        return metadata.level() <= self.global;
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{:<5} {} {}", record.level(), record.metadata().target(), record.args());
        }
    }

    fn flush(&self) {}
}


pub fn init() -> Result<(), SetLoggerError> {

    let mut global =  Level::Error;
    let mut targets = HashMap::new();


    if let Ok(l) = env::var("RUST_LOG") {
        for l in l.split(",") {
            let l : Vec<&str> = l.split("=").collect();
            match l.len() {
                1 => {
                    match l[0] {
                        "trace" => global = Level::Trace,
                        "warn"  => global = Level::Warn,
                        "debug" => global = Level::Debug,
                        "info"  => global = Level::Info,
                        "error" => global = Level::Error,
                        any =>  {targets.insert(format!("{}::", any), Level::Trace);},
                    }
                }
                2 => {
                    match l[1] {
                        "trace" => {targets.insert(format!("{}::", l[0]), Level::Trace);},
                        "warn"  => {targets.insert(format!("{}::", l[0]), Level::Warn);},
                        "debug" => {targets.insert(format!("{}::", l[0]), Level::Debug);},
                        "info"  => {targets.insert(format!("{}::", l[0]), Level::Info);},
                        "error" => {targets.insert(format!("{}::", l[0]), Level::Error);},
                        _ => (),
                    };
                }
                _ => (),
            }
        }
    }

    let t = Box::new(TinyLogger{
        global,
        targets,
    });
    log::set_logger(Box::leak(t))?;
    log::set_max_level(LevelFilter::Trace);
    Ok(())
}

