use sycamore::prelude::*;
use tracing::info;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlSelectElement};

use crate::{AppState, Device};

pub fn Controls<'a, 'b, G: Html>(
    ctx: BoundedScope<'a, 'b>,
    show_controls: &'a Signal<bool>,
) -> View<G> {
    let state = use_context::<AppState>(ctx);
    let binding = state.devices.get();
    let devices: &ReadSignal<Vec<Device>> =
        create_memo(ctx, move || binding.video_devices().cloned().collect());

    let visible = create_memo(ctx, || match *show_controls.get() {
        true => "visible",
        false => "invisible",
    });

    view! {ctx,
        div(class=format!("absolute top-1 P-2 {} border",visible)){
            select(
                class="bg-blue-500 rounded-sm p-1",
                on:click=|e: Event|{
                    let target=e.target().unwrap().unchecked_into::<HtmlSelectElement>();
                    let device_id=target.value();
                    info!("device_id: {}",&device_id);
                    state.device_id.set(device_id);
                }
            ) {
                Keyed (
                  iterable= devices,
                  key=|device| device.id.clone(),
                  view=|ctx, device| view! { ctx,option(value=device.id) {
                      (device.label)
                    }
                  }
                )
            }
        }
    }
}
