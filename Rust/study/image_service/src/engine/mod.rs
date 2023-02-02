use crate::Spec;
use image::ImageOutputFormat;

mod photon;
pub use photon::*;

pub trait Engine {
    // engine按照 spec进行一系列有序处理
    fn apply(&mut self, specs: &[Spec]);
    // 从 engine中生成图片
    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

pub trait SpecTransform<T> {
    fn transform(&mut self, op: T);
}
