use serde_json::json;
use sycamore::{futures, noderef, prelude::*};
use web_sys::HtmlVideoElement;
use webcam::VideoStream;

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

#[component]
fn Video<G: Html>(ctx: BoundedScope) -> View<G> {
    let video_ref = noderef::create_node_ref(ctx);
    futures::spawn_local_scoped(ctx, async move {
        let el = video_ref.get::<DomNode>().unchecked_into();
        let video_stream = VideoStream::new(el);
        video_stream
            .set_video_src(&json!({
                "auto":false, "video":{ "facingMode": "environment", "width": 640, "height": 480 },
            }))
            .await;
    });
    view! {ctx,
        div(ref=video_ref)
    }
    // view! {ctx,
    // div{
    //     video(
    //     style="border:1px slide",
    //     autoplay=false,
    //     width=640,
    //     height=480,
    //     src="https://imgs-qn.51miz.com/preview/video/00/00/14/33/V-143360-BCE1F72B.mp4",
    //     )
    // }
    // }
}
