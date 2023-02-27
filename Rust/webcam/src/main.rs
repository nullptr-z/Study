use serde_json::json;
use sycamore::prelude::*;
use tracing::Instrument;
use web_sys::{HtmlMediaElement, HtmlVideoElement, Node};
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
    let video_ref = create_node_ref(ctx);
    on_mount(ctx, || {
        let node = video_ref.get::<DomNode>();
        let el = node.unchecked_into::<HtmlVideoElement>();
        let video_stream = VideoStream::new(el);
        video_stream.set_video_src(&json!({
            "auto":false, "video":{ "facingMode":"environment","width":640, "height":480},
        }))
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
