use jni::{objects::JClass, sys::jint, JNIEnv};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[no_mangle]
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

/// Wrapper function for JNI. This follows JNI naming conventions
/// and data type conversions.
#[no_mangle]
pub unsafe extern "C" fn Java_com_reactnativepro_LibEnc_nativeAddNumbers(
    env: JNIEnv,
    class: JClass,
    a: jint,
    b: jint,
) -> jint {
    add_numbers(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
