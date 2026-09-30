use crate::il2cpp::{symbols::get_method_addr, types::*};

static mut CLASS: *mut Il2CppClass = 0 as _;
pub fn class() -> *mut Il2CppClass {
    unsafe { CLASS }
}

// public static Object Load(string path, Type type) { }
static mut LOAD_ADDR: usize = 0;
impl_addr_wrapper_fn!(Load, LOAD_ADDR, *mut Il2CppObject, path: *mut Il2CppString, type_object: *mut Il2CppObject);

pub fn init(UnityEngine_CoreModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_CoreModule, UnityEngine, Resources);

    unsafe {
        CLASS = Resources;
        LOAD_ADDR = get_method_addr(Resources, c"Load", 2);
    }

}
