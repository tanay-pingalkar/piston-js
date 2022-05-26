use quick_js::JsValue;

pub fn js_to_int(js: &JsValue) -> &i32 {
    match js {
        JsValue::Int(i) => i,
        _ => panic!("expected an integer"),
    }
}

pub fn i32vec_to_rgba(rgba: Vec<i32>) -> [f32; 4] {
    let rgba = [
        rgba[0] as f32,
        rgba[1] as f32,
        rgba[2] as f32,
        rgba[3] as f32,
    ];
    rgba
}

pub fn i32vec_to_f644(f64arr: Vec<i32>) -> [f64; 4] {
    let f64arr = [
        f64arr[0] as f64,
        f64arr[1] as f64,
        f64arr[2] as f64,
        f64arr[3] as f64,
    ];
    f64arr
}
