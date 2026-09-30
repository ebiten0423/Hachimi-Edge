use std::ptr::null_mut;

use crate::il2cpp::{api::il2cpp_object_new, symbols::get_method_addr, types::*};

static mut CLASS: *mut Il2CppClass = null_mut();
pub fn class() -> *mut Il2CppClass {
    unsafe { CLASS }
}

static mut CTOR_ADDR: usize = 0;
impl_addr_wrapper_fn!(_ctor, CTOR_ADDR, (),
    this: *mut Il2CppObject, width: i32, height: i32
);

pub fn new(width: i32, height: i32) -> *mut Il2CppObject {
    let this = il2cpp_object_new(class());
    _ctor(this, width, height);
    this
}

static mut READPIXELS_ADDR: usize = 0;
impl_addr_wrapper_fn!(ReadPixels, READPIXELS_ADDR, (), this: *mut Il2CppObject, source: Rect_t, dest_x: i32, dest_y: i32);

static mut APPLY_ADDR: usize = 0;
impl_addr_wrapper_fn!(Apply, APPLY_ADDR, (), this: *mut Il2CppObject);

pub fn init(UnityEngine_CoreModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_CoreModule, UnityEngine, Texture2D);

    unsafe {
        CLASS = Texture2D;
        CTOR_ADDR = get_method_addr(Texture2D, c".ctor", 2);
        READPIXELS_ADDR = get_method_addr(Texture2D, c"ReadPixels", 3);
        APPLY_ADDR = get_method_addr(Texture2D, c"Apply", 0);
    }
}
