mod enc;
use anyhow::Result;
use enc::AES;
use jni::{
    objects::{JByteArray, JClass},
    JNIEnv,
};
use std::fs;

// TODO: session and key management

#[no_mangle]
pub extern "system" fn Java_com_medwalletapp_RustBridgeModule_encryptByteArray<'local>(
    env: JNIEnv<'local>,
    _jclass: JClass,
    data: JByteArray<'local>,
) -> JByteArray<'local> {
    let input = env.convert_byte_array(&data).unwrap();
    let mut session = AES::new().unwrap();
    let cypher = session.encrypt(&input, None).unwrap();
    env.byte_array_from_slice(&cypher).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_com_medwalletapp_RustBridgeModule_decryptByteArray<'local>(
    env: JNIEnv<'local>,
    _jclass: JClass,
    cypher: JByteArray<'local>,
) -> JByteArray<'local> {
    let input = env.convert_byte_array(&cypher).unwrap();
    let session = AES::new().unwrap(); // FIX: This sessino will have a random key which will
                                       // cause a panic
    let cypher = session.decrypt(&input).unwrap();
    env.byte_array_from_slice(&cypher).unwrap()
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
        let mut session = AES::new().unwrap();
        let cypher = encrypt_file(&mut session, "raven.txt").unwrap();
        let decrypted = session.decrypt(&cypher).unwrap();
        let reference = fs::read("raven.txt").unwrap();

        assert_eq!(decrypted, reference);
    }
}
