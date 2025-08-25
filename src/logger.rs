use log::Level;
use std::io::Write;
use time::{OffsetDateTime, UtcOffset};

const RESET: &str = "\x1b[0m";
const GREY: &str = "\x1b[90m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const GREEN: &str = "\x1b[32m";
const ORANGE: &str = "\x1b[38;5;215m";
const CYAN: &str = "\x1b[36m";

fn level_color(l: Level) -> &'static str {
    match l {
        Level::Error => RED,
        Level::Warn => YELLOW,
        Level::Info => GREEN,
        Level::Debug => ORANGE,
        Level::Trace => CYAN,
    }
}

pub fn init() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .format(|buf, record| {
            let off = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
            let now = OffsetDateTime::now_utc().to_offset(off);
            let t = now.time();
            let ms = t.nanosecond() / 1_000_000;

            let time_s = format!(
                "{:02}:{:02}:{:02}:{:03}",
                t.hour(),
                t.minute(),
                t.second(),
                ms
            );
            let level_s = format!("[{}]", record.level());
            let lc = level_color(record.level());

            writeln!(
                buf,
                "{}{}{} {}{}{} {}",
                GREY,
                time_s,
                RESET,
                lc,
                level_s,
                RESET,
                record.args()
            )
        })
        .init();
}
