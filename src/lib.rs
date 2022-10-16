use jni::JNIEnv;

use jni::objects::{JClass, JString};

use jni::sys::jstring;

mod client;
mod async_try;

#[no_mangle]
pub extern "system" fn Java_com_eactative_ua_rn_HelloWorld_hello(
    env: JNIEnv,
    class: JClass,
    input: JString,
) -> jstring {
    let input: String = env
        .get_string(input)
        .expect("Couldn't get java string!")
        .into();

    let output = env.new_string(format!("Hello, {}!", input))
        .expect("Couldn't create java string!");
    output.into_inner()
}