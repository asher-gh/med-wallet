mod enc;
use anyhow::Result;
use enc::AES;
use jni::{objects::JClass, sys::jint, JNIEnv};
use std::fs;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn encrypt_file(session: &mut AES, file_path: &str) -> Result<Vec<u8>> {
    // TODO: session and key management
    let file = fs::read(file_path)?;
    let cypher = session.encrypt(&file, None)?;
    Ok(cypher)
}

#[no_mangle]
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

/// Wrapper function for JNI. This follows JNI naming conventions
/// and data type conversions.
#[no_mangle]
pub unsafe extern "C" fn Java_com_medwalletapp_RustBridgeModule_nativeAddNumbers(
    env: JNIEnv,
    jclass: JClass,
    a: jint,
    b: jint,
) -> jint {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]

    fn file_encryption() {
        let mut session = AES::new().unwrap();
        let cypher = encrypt_file(&mut session, "raven.txt").unwrap();
        let decrypted = session.decrypt(&cypher).unwrap();
        let reference = fs::read("raven.txt").unwrap();

        assert_eq!(decrypted, reference);
    }
}
