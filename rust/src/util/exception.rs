use jni::JNIEnv;

pub struct JavaException {
    class: String,
    message: String
}

impl JavaException {

    pub fn new_vad(message: String) -> JavaException {
        JavaException {
            class: "com/plasmoverse/vad/VadException".into(),
            message
        }
    }
}

pub trait JavaExceptions {

    fn throw_new_exception(&mut self, exception: JavaException);
}

impl<'local> JavaExceptions for JNIEnv<'local> {

    fn throw_new_exception(&mut self, exception: JavaException) {
        let _ = self.throw_new(exception.class, exception.message);
    }
}
