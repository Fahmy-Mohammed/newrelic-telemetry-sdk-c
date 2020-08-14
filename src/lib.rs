use newrelic_telemetry::attribute::{ToValue, Value};
use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

pub struct Sender {}
pub struct SpanBatch {}
pub struct Span {}
type Attributes = HashMap<String, Value>;

#[no_mangle]
pub extern "C" fn nrt_attributes_new() -> *mut Attributes {
    let attrs: HashMap<String, Value> = HashMap::new();
    Box::into_raw(Box::new(attrs))
}

fn nrt_attributes_set<T: ToValue>(
    attributes: *mut Attributes,
    key: *const c_char,
    value: T,
) -> bool {
    if attributes.is_null() || key.is_null() {
        return false;
    }

    if let Some(attrs) = unsafe { attributes.as_mut() } {
        if let Ok(key) = unsafe { CStr::from_ptr(key).to_str() } {
            attrs.insert(key.to_string(), value.to_attribute_value());
            return true;
        }
    }

    false
}

#[no_mangle]
pub extern "C" fn nrt_attributes_set_int(
    attributes: *mut Attributes,
    key: *const c_char,
    value: i64,
) -> bool {
    nrt_attributes_set(attributes, key, value)
}

#[no_mangle]
pub extern "C" fn nrt_attributes_set_uint(
    attributes: *mut Attributes,
    key: *const c_char,
    value: u64,
) -> bool {
    nrt_attributes_set(attributes, key, value)
}

#[no_mangle]
pub extern "C" fn nrt_attributes_set_double(
    attributes: *mut Attributes,
    key: *const c_char,
    value: f64,
) -> bool {
    nrt_attributes_set(attributes, key, value)
}

#[no_mangle]
pub extern "C" fn nrt_attributes_set_string(
    attributes: *mut Attributes,
    key: *const c_char,
    value: *const c_char,
) -> bool {
    if value.is_null() {
        return false;
    }

    if let Ok(value) = unsafe { CStr::from_ptr(value).to_str() } {
        nrt_attributes_set(attributes, key, value)
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn nrt_attributes_set_bool(
    attributes: *mut Attributes,
    key: *const c_char,
    value: bool,
) -> bool {
    nrt_attributes_set(attributes, key, value)
}

#[no_mangle]
pub extern "C" fn nrt_attributes_destroy(attributes: *mut *mut Attributes) {
    if attributes.is_null() {
        return;
    }
    unsafe { Box::from_raw(*attributes) };
}

#[no_mangle]
pub extern "C" fn nrt_span_new(
    id: *const c_char,
    trace_id: *const c_char,
    parent_id: *const c_char,
) -> *mut Span {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn nrt_span_set_name(span: *mut Span, name: *const c_char) -> bool {
    false
}

#[no_mangle]
pub extern "C" fn nrt_span_set_service_name(span: *mut Span, service_name: *const c_char) -> bool {
    false
}

#[no_mangle]
pub extern "C" fn nrt_span_set_timestamp(span: *mut Span, timestamp: u64) -> bool {
    false
}

#[no_mangle]
pub extern "C" fn nrt_span_set_duration(span: *mut Span, duration: u64) -> bool {
    false
}

#[no_mangle]
pub extern "C" fn nrt_span_set_attributes(
    span: *mut Span,
    attributes: *mut *mut Attributes,
) -> bool {
    if attributes.is_null() {
        false
    } else if span.is_null() {
        nrt_attributes_destroy(attributes);
        return false;
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn nrt_span_destroy(span: *mut *mut Span) {}

#[no_mangle]
pub extern "C" fn nrt_span_batch_new() -> *mut SpanBatch {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn nrt_span_batch_record(batch: *mut SpanBatch, span: *mut *mut Span) -> bool {
    true
}

#[no_mangle]
pub extern "C" fn nrt_span_batch_destroy(batch: *mut *mut SpanBatch) {}

#[no_mangle]
pub extern "C" fn nrt_sender_new(key: *const c_char) -> *mut Sender {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn nrt_sender_send(key: *const c_char, batch: *mut *mut SpanBatch) -> bool {
    false
}

#[no_mangle]
pub extern "C" fn nrt_sender_shutdown(sender: *mut *mut Sender) {}

#[no_mangle]
pub extern "C" fn nrt_sender_destroy(sender: *mut *mut Sender) {}
