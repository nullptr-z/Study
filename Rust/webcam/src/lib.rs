use tracing::info;
use web_sys::HtmlVideoElement;

pub struct VideoStream {
    el: HtmlVideoElement,
}

impl VideoStream {
    pub fn new(el: HtmlVideoElement) -> VideoStream {
        VideoStream { el }
    }

    pub fn set_video_src(&self, constraints: &serde_json::Value) {
        let window = web_sys::window().expect("window获取失败！");
        let navigator = window.navigator();
        let mediaDevices = navigator.media_devices().expect("mediaDevices get error");
        info!("devices (tracing_wasm): {:?}", mediaDevices);
        web_sys::console::log_1(&mediaDevices);
        // mediaDevices.get_user_media(constraints);
    }
}
