// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracing_subscriber::fmt::format::FmtSpan;

fn main() {
    let subscriber = tracing_subscriber::fmt()
        // .with_span_events(FmtSpan::FULL)
        .with_span_events(FmtSpan::NEW)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to setup tracing subscriber");
    viva_padel_lib::run()
}
