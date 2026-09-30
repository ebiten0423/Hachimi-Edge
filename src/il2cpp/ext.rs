use widestring::{Utf16Str, Utf16String};
use crate::il2cpp::api::il2cpp_string_new_utf16;
use crate::il2cpp::types::*;

pub trait StringExt {
    fn to_il2cpp_string(&self) -> *mut Il2CppString;
}

impl StringExt for str {
    fn to_il2cpp_string(&self) -> *mut Il2CppString {
        let text_utf16 = Utf16String::from_str(self);
        il2cpp_string_new_utf16(text_utf16.as_ptr(), text_utf16.len().try_into().unwrap())
    }
}

impl StringExt for String {
    fn to_il2cpp_string(&self) -> *mut Il2CppString {
        str::to_il2cpp_string(self)
    }
}

pub trait Il2CppStringExt {
    fn chars_ptr(&self) -> *const Il2CppChar;
    fn as_utf16str(&self) -> &Utf16Str;
}

impl Il2CppStringExt for Il2CppString {
    fn chars_ptr(&self) -> *const Il2CppChar {
        self.chars.as_ptr()
    }

    fn as_utf16str(&self) -> &Utf16Str {
        unsafe { Utf16Str::from_slice_unchecked(std::slice::from_raw_parts(self.chars.as_ptr(), self.length as usize)) }
    }

}

pub trait Il2CppObjectExt {
    fn klass(&self) -> *mut Il2CppClass;
}

impl Il2CppObjectExt for Il2CppObject {
    fn klass(&self) -> *mut Il2CppClass {
        unsafe { *self.__bindgen_anon_1.klass.as_ref() }
    }
}
