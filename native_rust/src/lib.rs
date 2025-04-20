use jni::{
    JNIEnv,
    objects::{JObject, JString},
};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_gix_rust_1native_App_helloFromRust<'l>(
    mut env: JNIEnv<'l>,
    _obj: JObject<'l>,
    name: JString<'l>,
) -> JString<'l> {
    let name: String = env.get_string(&name).expect("Cannot get string").into();

    let result = env
        .new_string(format!("Hello {}", name))
        .expect("Cannot create string");

    println!("Hello from Rust");

    result
}
