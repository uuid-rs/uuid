use jni::objects::{JByteArray, JClass, JObject, JValue};
use jni::sys::{jint, jlong, jlongArray, jshort};
use jni::JNIEnv;
use uuid::{Context, Timestamp, Uuid};

#[no_mangle]
pub extern "system" fn Java_io_github_honhimw_uuid_Context_init<'local>(
    mut env: JNIEnv<'local>,
    this: JObject<'local>,
    counter: jshort,
) {
    let context = Context::new(counter as u16);
    let ctx = Box::new(context);
    let ptr = Box::into_raw(ctx) as jlong;
    let _ = env.set_field(this, "contextPtr", "J", ptr.into());
}

#[no_mangle]
pub extern "system" fn Java_io_github_honhimw_uuid_Context_free<'local>(
    mut env: JNIEnv<'local>,
    mut this: JObject<'local>,
) {
    let ptr = match env.get_field(&mut this, "contextPtr", "J") {
        Ok(value) => value.j(),
        Err(e) => {
            let _ = env.throw_new("java/lang/IllegalStateException", e.to_string());
            return;
        },
    };
    match ptr {
        Ok(p) => {
            unsafe {
                let _ = Box::from_raw(p as *mut Context);
            }
            let _ = env.set_field(this, "contextPtr", "J", JValue::Long(0));
        }
        Err(e) => {
            let _ = env.throw_new("java/lang/IllegalStateException", e.to_string());
            return;
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_io_github_honhimw_uuid_InternalUuid_v1<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    counter_ptr: jlong,
    seconds: jlong,
    nanos: jint,
    node_id: JByteArray,
) -> jlongArray {
    let context = unsafe { &*(counter_ptr as *mut Context) };
    let ts = Timestamp::from_unix(&context, seconds as u64, nanos as u32);
    let mut buffer: [i8; 6] = [0; 6];
    let _ = env.get_byte_array_region(node_id, 0, &mut buffer);
    let buffer = buffer.map(|i| i as u8);
    let uuid = Uuid::new_v1(ts, &buffer);
    uuid_to_jlong_array(&mut env, &uuid)
}

#[no_mangle]
pub extern "system" fn Java_io_github_honhimw_uuid_InternalUuid_v3<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    m: jlong,
    l: jlong,
    name: JByteArray,
) -> jlongArray {
    match env.convert_byte_array(&name) {
        Ok(vec) => {
            let namespace = Uuid::from_u64_pair(m as u64, l as u64);
            let uuid = Uuid::new_v3(&namespace, vec.as_slice());
            uuid_to_jlong_array(&mut env, &uuid)
        }
        Err(e) => {
            let _ = env.throw_new("java/lang/IllegalStateException", e.to_string());
            jlongArray::default()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_io_github_honhimw_uuid_InternalUuid_v4<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
) -> jlongArray {
    let uuid = Uuid::new_v4();
    uuid_to_jlong_array(&mut env, &uuid)
}

#[no_mangle]
pub extern "system" fn Java_io_github_honhimw_uuid_InternalUuid_v5<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    m: jlong,
    l: jlong,
    name: JByteArray,
) -> jlongArray {
    match env.convert_byte_array(&name) {
        Ok(vec) => {
            let namespace = Uuid::from_u64_pair(m as u64, l as u64);
            let uuid = Uuid::new_v5(&namespace, vec.as_slice());
            uuid_to_jlong_array(&mut env, &uuid)
        }
        Err(e) => {
            let _ = env.throw_new("java/lang/IllegalStateException", e.to_string());
            jlongArray::default()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_io_github_honhimw_uuid_InternalUuid_v6<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    counter_ptr: jlong,
    seconds: jlong,
    nanos: jint,
    node_id: JByteArray,
) -> jlongArray {
    let context = unsafe { &*(counter_ptr as *mut Context) };
    let ts = Timestamp::from_unix(&context, seconds as u64, nanos as u32);
    let mut buffer: [i8; 6] = [0; 6];
    let _ = env.get_byte_array_region(node_id, 0, &mut buffer);
    let buffer = buffer.map(|i| i as u8);
    let uuid = Uuid::new_v6(ts, &buffer);
    uuid_to_jlong_array(&mut env, &uuid)
}

#[no_mangle]
pub extern "system" fn Java_io_github_honhimw_uuid_InternalUuid_v7<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    counter_ptr: jlong,
    seconds: jlong,
    nanos: jint,
) -> jlongArray {
    let context = unsafe { &*(counter_ptr as *mut Context) };
    let ts = Timestamp::from_unix(&context, seconds as u64, nanos as u32);
    let uuid = Uuid::new_v7(ts);
    uuid_to_jlong_array(&mut env, &uuid)
}

#[no_mangle]
pub extern "system" fn Java_io_github_honhimw_uuid_InternalUuid_v8<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    bytes: JByteArray,
) -> jlongArray {
    let mut buffer: [i8; 16] = [0; 16];
    let _ = env.get_byte_array_region(bytes, 0, &mut buffer);
    let buffer = buffer.map(|i| i as u8);
    let uuid = Uuid::new_v8(buffer);
    uuid_to_jlong_array(&mut env, &uuid)
}

#[no_mangle]
pub extern "system" fn Java_io_github_honhimw_uuid_InternalUuid_nowV1<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    node_id: JByteArray,
) -> jlongArray {
    let mut buffer: [i8; 6] = [0; 6];
    let _ = env.get_byte_array_region(node_id, 0, &mut buffer);
    let buffer = buffer.map(|i| i as u8);
    let uuid = Uuid::now_v1(&buffer);
    uuid_to_jlong_array(&mut env, &uuid)
}

#[no_mangle]
pub extern "system" fn Java_io_github_honhimw_uuid_InternalUuid_nowV6<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    node_id: JByteArray,
) -> jlongArray {
    let mut buffer: [i8; 6] = [0; 6];
    let _ = env.get_byte_array_region(node_id, 0, &mut buffer);
    let buffer = buffer.map(|i| i as u8);
    let uuid = Uuid::now_v6(&buffer);
    uuid_to_jlong_array(&mut env, &uuid)
}

#[no_mangle]
pub extern "system" fn Java_io_github_honhimw_uuid_InternalUuid_nowV7<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
) -> jlongArray {
    let uuid = Uuid::now_v7();
    uuid_to_jlong_array(&mut env, &uuid)
}

fn uuid_to_jlong_array<'local>(env: &mut JNIEnv<'local>, uuid: &Uuid) -> jlongArray {
    let (m, l) = uuid.as_u64_pair();

    match env.new_long_array(2) {
        Ok(long_array) => {
            let _ = env.set_long_array_region(&long_array, 0, &[m as i64, l as i64]);
            long_array.into_raw()
        }
        Err(e) => {
            let _ = env.throw_new("java/lang/IllegalStateException", e.to_string());
            jlongArray::default()
        }
    }
}
