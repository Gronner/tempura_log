// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use log::info;

use tempura_log::TempuraLogger;

fn main() {
    TempuraLogger::init();

    info!("Test log");
}
