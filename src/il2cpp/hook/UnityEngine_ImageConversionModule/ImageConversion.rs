use crate::il2cpp::{symbols::get_method_addr, types::*};

static mut ENCODETOPNG_ADDR: usize = 0;
impl_addr_wrapper_fn!(EncodeToPNG, ENCODETOPNG_ADDR, *mut Il2CppArraySize, tex: *mut Il2CppObject);

pub fn init(UnityEngine_ImageConversionModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_ImageConversionModule, UnityEngine, ImageConversion);

    unsafe {
        ENCODETOPNG_ADDR = get_method_addr(ImageConversion, c"EncodeToPNG", 1);
    }
}
