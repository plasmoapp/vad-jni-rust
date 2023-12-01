use eyre::Report;
use jni::JNIEnv;
use jni::objects::{JObject};
use jni::sys::jlong;

pub trait JavaPointers<T> {

    fn into_jlong_pointer(self) -> jlong;

    unsafe fn from_jlong_pointer<'a>(pointer: jlong) -> &'a mut T {
        &mut *(pointer as *mut T)
    }
}

pub fn get_pointer_from_field(env: &mut JNIEnv, object: &JObject, field: String) -> Result<jlong, Report> {
    let field = env.get_field(object, field, "J")?;
    let pointer = field.j()?;

    Ok(pointer)
}
