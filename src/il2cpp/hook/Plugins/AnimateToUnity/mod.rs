use crate::il2cpp::types::Il2CppImage;
pub mod AnRoot;
pub fn init(image: *const Il2CppImage) { AnRoot::init(image); }
