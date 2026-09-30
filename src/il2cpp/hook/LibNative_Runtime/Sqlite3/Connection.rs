use crate::il2cpp::{api::{il2cpp_object_new, il2cpp_runtime_object_init}, symbols::get_method_addr, types::*};

static mut CLASS: *mut Il2CppClass = std::ptr::null_mut();
pub fn class() -> *mut Il2CppClass { unsafe { CLASS } }
pub fn new() -> *mut Il2CppObject {
    let object = il2cpp_object_new(class());
    il2cpp_runtime_object_init(object);
    object
}

static mut QUERY_ADDR: usize = 0;
impl_addr_wrapper_fn!(Query, QUERY_ADDR, *mut Il2CppObject, this: *mut Il2CppObject, sql: *const Il2CppString);
static mut OPEN_ADDR: usize = 0;
impl_addr_wrapper_fn!(Open, OPEN_ADDR, bool, this: *mut Il2CppObject, file_name: *mut Il2CppString, vfs_name: *mut Il2CppString, key: *mut Il2CppArray, cipher_type: i32);
static mut CLOSEDB_ADDR: usize = 0;
impl_addr_wrapper_fn!(CloseDB, CLOSEDB_ADDR, (), this: *mut Il2CppObject);

pub fn init(image: *const Il2CppImage) {
    get_class_or_return!(image, "LibNative.Sqlite3", Connection);
    unsafe {
        CLASS = Connection;
        QUERY_ADDR = get_method_addr(Connection, c"Query", 1);
        OPEN_ADDR = get_method_addr(Connection, c"Open", 4);
        CLOSEDB_ADDR = get_method_addr(Connection, c"CloseDB", 0);
    }
}
