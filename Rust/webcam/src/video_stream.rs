use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, HtmlVideoElement, MediaStream, MediaStreamConstraints};

use crate::Devices;

pub struct VideoStream {
    el: HtmlVideoElement,
}

impl VideoStream {
    pub fn new(el: HtmlVideoElement) -> VideoStream {
        VideoStream { el }
    }

    pub async fn set_video_src(&self, constraints_serde_json: &serde_json::Value) {
        let media_devices = Devices::get_media_devices().await;

        let mut constraints = MediaStreamConstraints::new();
        constraints.video(
            &serde_wasm_bindgen::to_value(constraints_serde_json)
                .expect("serde_json::Value to JsValue Error"),
        );
        constraints.audio(&false.into());

        let with_constraints = media_devices
            .get_user_media_with_constraints(&constraints)
            .unwrap();
        let media_jsv = JsFuture::from(with_constraints).await.unwrap();
        let media_stream = media_jsv.unchecked_into::<MediaStream>();
        // let media_stream = MediaStream::new_with_tracks(&media_jsv);
        self.el.set_src_object(Some(&media_stream));
    }
}
