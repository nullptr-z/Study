use serde_json::json;
use sycamore::{futures, noderef, prelude::*};
use tracing::info;

use crate::{Controls, Devices, VideoStream};

#[component]
pub fn Video<G: Html>(ctx: BoundedScope) -> View<G> {
    let video_ref = noderef::create_node_ref(ctx);
    sycamore::web::on_mount(ctx, move || {
        let el = video_ref.get::<DomNode>().unchecked_into();
        futures::spawn_local_scoped(ctx, async move {
            let video_stream = VideoStream::new(el);
            video_stream
                .set_video_src(&json!({
                    "audio": true,
                    "video":{ "facingMode": "environment", "width": 1280, "height": 720 },
                }))
                .await;

            // let devices = Devices::load().await;
            // info!("devices:{:?}", devices);
        });
    });

    view! {ctx,
    div(class="relative"){
        video(
            ref=video_ref,
            class="rounded-lg",
            width=640,
            height=480,
            autoplay=true,
            // src="https://imgs-qn.51miz.com/preview/video/00/00/14/33/V-143360-BCE1F72B.mp4",
        )
        Controls()
        }
    }
}
