use fern::Dispatch;
use log::LevelFilter;
use std::io;
use web_sys;

pub fn init_logger() {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                current_time(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Debug)
        .chain(Box::new(WebConsole::new()) as Box<dyn io::Write + Send>)
        .apply()
        .unwrap();
}

struct WebConsole {
    buffer: String,
}

impl WebConsole {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }
}

unsafe impl Send for WebConsole {}

impl io::Write for WebConsole {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Ok(s) = std::str::from_utf8(buf) {
            self.buffer.push_str(s);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        let s = self.buffer.trim_end_matches('\n');
        web_sys::console::log_1(&s.into());
        self.buffer.clear();
        Ok(())
    }
}

fn current_time() -> String {
    let d = js_sys::Date::new_0();
    let h = d.get_hours();
    let m = d.get_minutes();
    let s = d.get_seconds();
    let ms = d.get_milliseconds();
    format!("{:02}:{:02}:{:02}.{:04}", h, m, s, ms)
}
