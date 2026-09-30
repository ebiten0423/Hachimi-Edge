use crate::il2cpp::{symbols::get_method_addr, types::*};

static mut LINEHEADWRAP_ADDR: usize = 0;
impl_addr_wrapper_fn!(LineHeadWrap, LINEHEADWRAP_ADDR, *mut Il2CppString, text: *mut Il2CppString, line_char_count: i32);

pub fn init(image: *const Il2CppImage) {
    get_class_or_return!(image, Gallop, GallopUtil);
    unsafe { LINEHEADWRAP_ADDR = get_method_addr(GallopUtil, c"LineHeadWrap", 2); }
}
