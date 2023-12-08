mod enc;
use anyhow::Result;
use enc::AES;
use jni::{
    objects::{JByteArray, JClass, JObject},
    sys::jlong,
    JNIEnv,
};
use std::fs;

// TODO: session and key management

#[no_mangle]
pub unsafe extern "system" fn Java_com_medwalletapp_RustBridgeModule_encryptByteArray<'local>(
    env: JNIEnv<'local>,
    _jclass: JClass,
    aes_ptr: jlong,
    data: JByteArray<'local>,
) -> JByteArray<'local> {
    let aes = &mut *(aes_ptr as *mut AES);
    let input = env.convert_byte_array(&data).unwrap();
    let cypher = aes.encrypt(&input, None).unwrap();
    env.byte_array_from_slice(&cypher).unwrap()
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_medwalletapp_RustBridgeModule_decryptByteArray<'local>(
    env: JNIEnv<'local>,
    _jclass: JClass,
    aes_ptr: jlong,
    cypher: JByteArray<'local>,
) -> JByteArray<'local> {
    let input = env.convert_byte_array(&cypher).unwrap();
    let aes = &mut *(aes_ptr as *mut AES);
    let decrypted = aes.decrypt(&input).unwrap();
    env.byte_array_from_slice(&decrypted).unwrap()
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_medwalletapp_RustBridgeModule_createAES<'local>(
    env: JNIEnv<'local>,
    _jclass: JClass,
    callback: JObject,
) -> jlong {
    // this is to avoid GC for callback
    let _global_ref = env.new_global_ref(callback).unwrap();
    let aes = AES::new();

    Box::into_raw(Box::new(aes)) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_medwalletapp_RustBridgeModule_destroyAES(
    _env: JNIEnv,
    _class: JClass,
    aes_ptr: jlong,
) {
    let _boxed_counter = Box::from_raw(aes_ptr as *mut AES);
}

pub fn encrypt_file(session: &mut AES, file_path: &str) -> Result<Vec<u8>> {
    let file = fs::read(file_path)?;
    let cypher = session.encrypt(&file, None)?;
    Ok(cypher)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn file_encryption() {
        let mut aes = AES::new().unwrap();
        let cypher = encrypt_file(&mut aes, "raven.txt").unwrap();
        let decrypted = aes.decrypt(&cypher).unwrap();
        let reference = fs::read("raven.txt").unwrap();

        assert_eq!(decrypted, reference);
    }
}
