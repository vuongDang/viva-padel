mod app;
mod book_court;

use app::*;
use leptos::*;


fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    // fmt()
    //     .with_writer(
    //         // To avoide trace events in the browser from showing their
    //         // JS backtrace, which is very annoying, in my opinion
    //         MakeConsoleWriter::default().map_trace_level_to(tracing::Level::DEBUG),
    //     )
    //     // For some reason, if we don't do this in the browser, we get
    //     // a runtime error.
    //     .without_time()
    //     // .with_span_events(FmtSpan::NEW)
    //     .init();

    mount_to_body(|| {
        view! { <App /> }
    })
}
