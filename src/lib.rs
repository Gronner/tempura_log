// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use log::{LevelFilter, Log};

pub struct TempuraLogger;

impl TempuraLogger {
    pub fn init() {
        log::set_boxed_logger(Box::new(TempuraLogger))
            .map(|()| log::set_max_level(LevelFilter::Info))
            .unwrap();
    }
}

impl Log for TempuraLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        println!(
            "[{}] {}: {}",
            record.level(),
            record.target(),
            record.args()
        );
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;

    #[test]
    fn logging_is_executed() {
        TempuraLogger::init();
        info!("Test log");
        log::logger().flush();
    }
}
