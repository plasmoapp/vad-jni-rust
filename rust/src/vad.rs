use jni::JNIEnv;
use jni::objects::{JClass, JObject, JShortArray, JValue};
use jni::sys::{jboolean, jint, jlong, JNI_FALSE, jshort};
use webrtc_vad::{SampleRate, Vad, VadMode};

use crate::util::exception::{JavaException, JavaExceptions};
use crate::util::into_exception::ErrIntoException;
use crate::util::pointer::{get_pointer_from_field, JavaPointers};
use crate::vad_container::VadContainer;

#[no_mangle]
pub extern "system" fn Java_com_plasmoverse_vad_Vad_createNative(
    mut env: JNIEnv,
    _class: JClass,
    sample_rate: jint,
    mode: jint,
) -> jlong {
    match create_vad(sample_rate, mode) {
        Ok(pointer) => pointer,
        Err(exception) => {
            env.throw_new_exception(exception);
            0
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_plasmoverse_vad_Vad_resetNative(
    mut env: JNIEnv,
    vad: JObject
) {
    match vad_reset(&mut env, vad) {
        Ok(pointer) => pointer,
        Err(exception) => {
            env.throw_new_exception(exception);
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_plasmoverse_vad_Vad_closeNative(
    mut env: JNIEnv,
    vad: JObject
) {
    match vad_close(&mut env, vad) {
        Ok(pointer) => pointer,
        Err(exception) => {
            env.throw_new_exception(exception);
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_plasmoverse_vad_Vad_setModeNative(
    mut env: JNIEnv,
    encoder: JObject,
    bitrate: jint
) {
    match vad_set_mode(&mut env, encoder, bitrate) {
        Ok(decoded) => decoded,
        Err(exception) => {
            env.throw_new_exception(exception);
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_plasmoverse_vad_Vad_setSampleRateNative(
    mut env: JNIEnv,
    encoder: JObject,
    sample_rate: jint
) {
    match vad_set_sample_rate(&mut env, encoder, sample_rate) {
        Ok(decoded) => decoded,
        Err(exception) => {
            env.throw_new_exception(exception);
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_plasmoverse_vad_Vad_isVoiceSegmentNative<'local>(
    mut env: JNIEnv<'local>,
    vad: JObject<'local>,
    samples: JShortArray<'local>
) -> jboolean {
    match vad_is_voice_segment(&mut env, vad, samples) {
        Ok(decoded) => decoded,
        Err(exception) => {
            env.throw_new_exception(exception);
            JNI_FALSE
        }
    }
}

fn create_vad(
    sample_rate: jint,
    mode: jint,
) -> Result<jlong, JavaException> {
    let vad_sample_rate = SampleRate::try_from(sample_rate)
        .err_into_vad_exception("Failed to create VAD instance")?;

    let vad_mode = match mode {
        1 => VadMode::LowBitrate,
        2 => VadMode::Aggressive,
        3 => VadMode::VeryAggressive,
        _ => VadMode::Quality
    };

    let vad = Vad::new_with_rate_and_mode(vad_sample_rate, vad_mode);

    let encoder_container = VadContainer {
        vad,
        sample_rate,
        vad_mode: mode
    };

    Ok(encoder_container.into_jlong_pointer())
}

unsafe fn get_vad_container<'local>(
    env: &mut JNIEnv,
    encoder: &JObject
) -> Result<&'local mut VadContainer, JavaException> {
    let pointer = get_pointer_from_field(env, encoder, "pointer".into())
        .err_into_vad_exception("Failed to get a pointer from the java object")?;

    Ok(VadContainer::from_jlong_pointer(pointer))
}

unsafe fn vad_reset(
    env: &mut JNIEnv,
    encoder: JObject
) -> Result<(), JavaException> {
    let container = get_vad_container(env, &encoder)?;

    container.vad.reset();

    let vad_sample_rate = SampleRate::try_from(container.sample_rate)
        .err_into_vad_exception("Failed to create VAD instance")?;

    let vad_mode = match container.vad_mode {
        1 => VadMode::LowBitrate,
        2 => VadMode::Aggressive,
        3 => VadMode::VeryAggressive,
        _ => VadMode::Quality
    };

    container.vad.set_sample_rate(vad_sample_rate);
    container.vad.set_mode(vad_mode);

    Ok(())
}

unsafe fn vad_close(
    env: &mut JNIEnv,
    encoder: JObject
) -> Result<(), JavaException> {
    let pointer = get_pointer_from_field(env, &encoder, "pointer".into())
        .err_into_vad_exception("Failed to get a pointer from the java object")?;

    let _container = Box::from_raw(pointer as *mut VadContainer);
    env.set_field(&encoder, "pointer", "J", JValue::from(0 as jlong))
        .err_into_vad_exception("Failed to reset pointer")?;

    Ok(())
}

unsafe fn vad_is_voice_segment<'local>(
    env: &mut JNIEnv<'local>,
    encoder: JObject<'local>,
    samples: JShortArray<'local>
) -> Result<jboolean, JavaException> {
    let container = get_vad_container(env, &encoder)?;

    let samples_length = env.get_array_length(&samples)
        .err_into_vad_exception("Failed to get samples array length")?
        as usize;

    let mut samples_vec = vec![0i16 as jshort; samples_length];

    env.get_short_array_region(samples, 0, &mut samples_vec)
        .err_into_vad_exception("Failed to copy samples to rust vec")?;

    let result = container.vad.is_voice_segment(&samples_vec)
        .map_err(|_error| JavaException::new_vad(
            format!("Failed to calculate VAD")
        ))?;

    Ok(result as jboolean)
}

unsafe fn vad_set_mode(
    env: &mut JNIEnv,
    vad: JObject,
    mode: jint
) -> Result<(), JavaException> {
    let container = get_vad_container(env, &vad)?;

    let vad_mode = match mode {
        1 => VadMode::LowBitrate,
        2 => VadMode::Aggressive,
        3 => VadMode::VeryAggressive,
        _ => VadMode::Quality
    };

    container.vad.set_mode(vad_mode);
    container.vad_mode = mode;

    Ok(())
}

unsafe fn vad_set_sample_rate(
    env: &mut JNIEnv,
    vad: JObject,
    sample_rate: jint
) -> Result<(), JavaException> {
    let container = get_vad_container(env, &vad)?;

    let vad_sample_rate = SampleRate::try_from(container.sample_rate)
        .err_into_vad_exception("Failed to create VAD instance")?;

    container.vad.set_sample_rate(vad_sample_rate);
    container.vad_mode = sample_rate;

    Ok(())
}
