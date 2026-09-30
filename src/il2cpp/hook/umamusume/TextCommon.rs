use crate::{core::Hachimi, il2cpp::{hook::UnityEngine_UI::Text, symbols::{get_method_addr, get_type_object_for_class}, types::*}};

static mut TYPE_OBJECT: *mut Il2CppObject = 0 as _;
pub fn type_object() -> *mut Il2CppObject {
    unsafe { TYPE_OBJECT }
}

static mut GET_IS_ACTIVE_IN_HIERARCHY_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_IsActiveInHierarchy, GET_IS_ACTIVE_IN_HIERARCHY_ADDR, bool, this: *mut Il2CppObject);

static mut SET_FONTCOLOR_ADDR: usize = 0;
impl_addr_wrapper_fn!(set_FontColor, SET_FONTCOLOR_ADDR, (), this: *mut Il2CppObject, value: i32);

static mut SET_OUTLINESIZE_ADDR: usize = 0;
impl_addr_wrapper_fn!(set_OutlineSize, SET_OUTLINESIZE_ADDR, (), this: *mut Il2CppObject, value: i32);

static mut UPDATEOUTLINE_ADDR: usize = 0;
impl_addr_wrapper_fn!(UpdateOutline, UPDATEOUTLINE_ADDR, (), this: *mut Il2CppObject);

static mut SET_OUTLINECOLOR_ADDR: usize = 0;
impl_addr_wrapper_fn!(set_OutlineColor, SET_OUTLINECOLOR_ADDR, (), this: *mut Il2CppObject, value: i32);

static mut REBUILDOUTLINE_ADDR: usize = 0;
impl_addr_wrapper_fn!(RebuildOutline, REBUILDOUTLINE_ADDR, (), this: *mut Il2CppObject);

type AwakeFn = extern "C" fn(this: *mut Il2CppObject);
extern "C" fn Awake(this: *mut Il2CppObject) {
    get_orig_fn!(Awake, AwakeFn)(this);
    if Hachimi::instance().config.load().replace_to_builtin_font {
        Text::AssignDefaultFont(this);
    }
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, TextCommon);

    let Awake_addr = get_method_addr(TextCommon, c"Awake", 0);
    new_hook!(Awake_addr, Awake);

    unsafe {
        TYPE_OBJECT = get_type_object_for_class(TextCommon);
        GET_IS_ACTIVE_IN_HIERARCHY_ADDR = get_method_addr(TextCommon, c"get_IsActiveInHierarchy", 0);
        SET_FONTCOLOR_ADDR = get_method_addr(TextCommon, c"set_FontColor", 1);
        SET_OUTLINESIZE_ADDR = get_method_addr(TextCommon, c"set_OutlineSize", 1);
        UPDATEOUTLINE_ADDR = get_method_addr(TextCommon, c"UpdateOutline", 0);
        SET_OUTLINECOLOR_ADDR = get_method_addr(TextCommon, c"set_OutlineColor", 1);
        REBUILDOUTLINE_ADDR = get_method_addr(TextCommon, c"RebuildOutline", 0);
    }
}
