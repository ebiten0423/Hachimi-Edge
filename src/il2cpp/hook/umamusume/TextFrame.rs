

use crate::il2cpp::{symbols::get_method_addr, types::*};

static mut GET_TEXTLABEL_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_TextLabel, GET_TEXTLABEL_ADDR, *mut Il2CppObject, this: *mut Il2CppObject);

static mut GET_NAMELABEL_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_NameLabel, GET_NAMELABEL_ADDR, *mut Il2CppObject, this: *mut Il2CppObject);

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, TextFrame);

    unsafe {
        GET_TEXTLABEL_ADDR = get_method_addr(TextFrame, c"get_TextLabel", 0);
        GET_NAMELABEL_ADDR = get_method_addr(TextFrame, c"get_NameLabel", 0);
    }
}