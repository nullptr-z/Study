use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

mod controls;
mod video;

// @1
use crate::{set_window_decorations, AppState};
pub use controls::*;
use tracing::{info, Event};
pub use video::Video;

#[component]
pub async fn App<G: Html>(ctx: BoundedScope<'_, '_>) -> View<G> {
    let state = AppState::new().await;
    provide_context(ctx, state);

    window_event_lisener(
        ctx,
        "resize",
        Box::new(move || {
            let state = use_context::<AppState>(ctx);

            let window = web_sys::window().unwrap();
            let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
            let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
            // console::log_2(&width.into(), &height.into());
            state.dimensions.set((width, height));
        }),
    );

    window_event_lisener(
        ctx,
        "mouseover",
        Box::new(move || {
            spawn_local(async move {
                info!("window mouseover");
                set_window_decorations(true).await;
            })
        }),
    );

    window_event_lisener(
        ctx,
        "mouseout",
        Box::new(move || {
            spawn_local(async move {
                info!("window mouseover");
                set_window_decorations(false).await;
            })
        }),
    );

    view! {ctx,
        Video()
    }
}

fn window_event_lisener<'a, 'b>(
    ctx: BoundedScope<'a, 'b>,
    event: &str,
    callback: Box<dyn FnMut() + 'a>,
) {
    let window = web_sys::window().unwrap();
    // into_js_value内部 mem::forget(self)，使JsValue 不受 Rust 生命周期影响--Rust 不回收这些内存
    // 需要写代码时注意删除这些 mem, 以防内存泄漏
    let handel: Box<dyn FnMut() + 'static> = unsafe { std::mem::transmute(callback) };
    let callback = Closure::new(handel).into_js_value();
    window
        .add_event_listener_with_callback(event, callback.unchecked_ref())
        .unwrap();

    on_cleanup(ctx, move || drop(callback));
}
