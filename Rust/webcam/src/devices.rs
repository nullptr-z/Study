use std::ops::Deref;

use tracing::info;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{MediaDeviceInfo, MediaDeviceKind, MediaDevices, MediaStreamConstraints};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Devices(Vec<Device>);

impl Deref for Devices {
    type Target = Vec<Device>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Iterator for Devices {
    type Item = Device;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl Devices {
    pub async fn load() -> Self {
        let media_devices = Self::get_media_devices().await;
        let list_media_devices = JsFuture::from(
            media_devices
                .enumerate_devices()
                .expect("enumerate_devices 调用出错"),
        )
        .await
        .unwrap();

        Self::from(&list_media_devices)
    }

    pub async fn get_media_devices() -> MediaDevices {
        // MediaStreamConstraints::new().video(&true.into());
        let window = web_sys::window().expect("window获取失败！");
        let navigator = window.navigator();
        let media_devices = navigator.media_devices().expect("mediaDevices get error");

        JsFuture::from(
            media_devices
                .get_user_media_with_constraints(&MediaStreamConstraints::new().video(&true.into()))
                .unwrap(),
        )
        .await
        .unwrap();

        media_devices
    }

    pub fn video_devices(&self) -> impl Iterator<Item = &Device> {
        self.iter_by_kind(MediaDeviceKind::Videoinput)
    }

    pub fn audio_devices(&self) -> impl Iterator<Item = &Device> {
        self.iter_by_kind(MediaDeviceKind::Audioinput)
    }

    fn iter_by_kind(&self, kind: MediaDeviceKind) -> impl Iterator<Item = &Device> {
        self.iter().filter(move |flt| flt.kind == kind)
    }
}

impl From<&JsValue> for Devices {
    fn from(value: &JsValue) -> Self {
        match js_sys::try_iter(value) {
            Ok(Some(item)) => {
                let devices = item
                    .into_iter()
                    .filter(|flt| flt.is_ok())
                    .map(|v| Device::from(v.unwrap()))
                    .collect::<Vec<_>>();
                Devices(devices)
            }
            _ => Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Device {
    pub kind: MediaDeviceKind,
    pub label: String,
    pub id: String,
}

impl From<JsValue> for Device {
    fn from(value: JsValue) -> Self {
        let device = value.unchecked_into::<MediaDeviceInfo>();
        Device {
            kind: device.kind(),
            label: device.label(),
            id: device.device_id(),
        }
    }
}
