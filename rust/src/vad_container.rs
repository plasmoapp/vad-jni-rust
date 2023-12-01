use jni::sys::jlong;
use webrtc_vad::Vad;

use crate::util::pointer::JavaPointers;

pub struct VadContainer {
    pub vad: Vad,
    pub sample_rate: i32,
    pub vad_mode: i32,
}

impl JavaPointers<VadContainer> for VadContainer {
    fn into_jlong_pointer(self) -> jlong {
        Box::into_raw(Box::new(self)) as jlong
    }
}
