use crate::il2cpp::{api::{il2cpp_class_get_type, il2cpp_type_get_object}, symbols::{get_field_from_name, get_field_object_value}, types::*};

static mut TYPE_OBJECT: *mut Il2CppObject = std::ptr::null_mut();
pub fn type_object() -> *mut Il2CppObject { unsafe { TYPE_OBJECT } }

static mut _TOPOBJECT_FIELD: *mut FieldInfo = std::ptr::null_mut();
pub fn get__topObject(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { _TOPOBJECT_FIELD })
}

pub fn init(image: *const Il2CppImage) {
    get_class_or_return!(image, AnimateToUnity, AnRoot);
    unsafe {
        TYPE_OBJECT = il2cpp_type_get_object(il2cpp_class_get_type(AnRoot));
        _TOPOBJECT_FIELD = get_field_from_name(AnRoot, c"_topObject");
    }
}
