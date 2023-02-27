use sycamore::prelude::*;
use webcam::Video;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    sycamore::render(|ctx| {
        view! {ctx,
                div(class="container p-2"){
                    Video()
            }
        }
    })
}
