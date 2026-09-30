use crate::il2cpp::{symbols::get_method_addr, types::*};

static mut GETTEXT_ADDR: usize = 0;
impl_addr_wrapper_fn!(GetText, GETTEXT_ADDR, *mut Il2CppString, this: *mut Il2CppObject, idx: i32);
static mut GETINT_ADDR: usize = 0;
impl_addr_wrapper_fn!(GetInt, GETINT_ADDR, i32, this: *mut Il2CppObject, idx: i32);
static mut STEP_ADDR: usize = 0;
impl_addr_wrapper_fn!(Step, STEP_ADDR, bool, this: *mut Il2CppObject);
static mut DISPOSE_ADDR: usize = 0;
impl_addr_wrapper_fn!(Dispose, DISPOSE_ADDR, (), this: *mut Il2CppObject);

pub fn init(image: *const Il2CppImage) {
    get_class_or_return!(image, "LibNative.Sqlite3", Query);
    unsafe {
        GETTEXT_ADDR = get_method_addr(Query, c"GetText", 1);
        GETINT_ADDR = get_method_addr(Query, c"GetInt", 1);
        STEP_ADDR = get_method_addr(Query, c"Step", 0);
        DISPOSE_ADDR = get_method_addr(Query, c"Dispose", 0);
    }
}
