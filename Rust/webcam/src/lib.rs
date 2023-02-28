mod components;
mod devices;
mod video_stream;

pub use components::*;
pub use devices::*;
use sycamore::prelude::*;
pub use video_stream::VideoStream;

#[derive(Debug)]
pub struct AppState {
    pub device_id: RcSignal<String>,
    pub devices: RcSignal<Devices>,
}

impl AppState {
    pub async fn new() -> Self {
        let device_id = create_rc_signal("".into());
        let devices = create_rc_signal(Devices::load().await);
        Self { device_id, devices }
    }
}
