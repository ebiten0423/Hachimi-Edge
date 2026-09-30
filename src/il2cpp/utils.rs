

use crate::il2cpp::{ext::{Il2CppObjectExt, Il2CppStringExt}, hook::UnityEngine_CoreModule::{Component, RectTransform}, types::*};

use super::{
    api::{il2cpp_class_get_fields, il2cpp_class_is_enum, il2cpp_field_get_flags, il2cpp_field_get_name},
    symbols::{get_assembly_image, get_class, get_method_addr_cached}
};

#[allow(dead_code)]
pub fn print_stack_trace() {
    let mscorlib = get_assembly_image(c"mscorlib.dll").expect("mscorlib");
    let environment_class = get_class(mscorlib, c"System", c"Environment").expect("System.Environment");
    let get_fn_addr = get_method_addr_cached(environment_class, c"get_StackTrace", 0);
    let get_fn: extern "C" fn() -> *mut Il2CppString = unsafe { std::mem::transmute(get_fn_addr) };
    debug!("{}", unsafe { (*get_fn()).as_utf16str() });
}

/// Changes the "active area" of the GameObject a Component is part of.
/// Numbers <=0 skip that axis.
pub fn adjust_transform_size(component: *mut Il2CppObject, width: f32, height: f32) {
    let transform = Component::get_transform(component);
    if unsafe { (*transform).klass() } == RectTransform::class() {
        if width > 0.0 {
            RectTransform::SetSizeWithCurrentAnchors(transform, RectTransform::Axis::Horizontal, width);
        }
        if height > 0.0 {
            RectTransform::SetSizeWithCurrentAnchors(transform, RectTransform::Axis::Vertical, height);
        }
    }
}

pub fn umamusume_enum_options(class_name: &std::ffi::CStr) -> Vec<String> {
    let mut options = Vec::new();
    let Ok(image) = get_assembly_image(c"umamusume.dll") else { return options };
    let Ok(klass) = get_class(image, c"Gallop", class_name) else { return options };

    if !il2cpp_class_is_enum(klass) { return options; }

    let mut iter: *mut std::ffi::c_void = std::ptr::null_mut();
    loop {
        let field = il2cpp_class_get_fields(klass, &mut iter);
        if field.is_null() { break; }
        let attrs = il2cpp_field_get_flags(field);
        if (attrs & 0x0040) != 0 {
            let name_ptr = il2cpp_field_get_name(field);
            if !name_ptr.is_null() {
                let name = unsafe { std::ffi::CStr::from_ptr(name_ptr) };
                if let Ok(s) = name.to_str() {
                    options.push(s.to_string());
                }
            }
        }
    }
    options
}
