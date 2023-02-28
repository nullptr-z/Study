use sycamore::prelude::*;
use tracing::info;

use crate::{AppState, Device};

pub fn Controls<G: Html>(ctx: BoundedScope) -> View<G> {
    let state = use_context::<AppState>(ctx);
    let binding = state.devices.get();
    let devices: &ReadSignal<Vec<Device>> =
        create_memo(ctx, move || binding.video_devices().cloned().collect());

    view! {ctx,
        div(class="absolute  bottom-2 p-5 "){
            select(class="bg-blue-500 rounded-sm p-1" ) {
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
