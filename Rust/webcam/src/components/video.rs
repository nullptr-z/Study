use serde_json::json;
use sycamore::{futures, noderef, prelude::*};
use tracing::info;

use crate::{AppState, Controls, VideoStream};

#[component]
pub fn Video<G: Html>(ctx: BoundedScope) -> View<G> {
    let state = use_context::<AppState>(ctx);
    let video_ref = noderef::create_node_ref(ctx);
    let constraints = create_selector(ctx, || match state.device_id.get().as_str() {
        "" => {
            json!({
                "facingMode": "user",
            })
        }
        id => {
            json!({"deviceId": { "exact": id},
            // "width": { "exact":state.get_width()},
            // "height": { "exact":state.get_height()}
            })
        }
    });
    create_effect(ctx, move || {
        constraints.track();
        sycamore::web::on_mount(ctx, move || {
            let el = video_ref.get::<DomNode>().unchecked_into();
            futures::spawn_local_scoped(ctx, async move {
                let video_stream = VideoStream::new(el);
                video_stream.set_video_src(&constraints.get()).await;
            });
        });
    });

    let show_controls = create_signal(ctx, true);

    view! {ctx,
    div(
        class="relative",
        on:mouseover=move|_| show_controls.set(true),
        on:mouseout=move|_| show_controls.set(false),
    ){
        video(
            ref=video_ref,
            class="rounded-lg",
            width=state.get_width(),
            height=state.get_height(),
            autoplay=true,
            // src="https://imgs-qn.51miz.com/preview/video/00/00/14/33/V-143360-BCE1F72B.mp4",
        )
        Controls(show_controls)
        }
    }
}
