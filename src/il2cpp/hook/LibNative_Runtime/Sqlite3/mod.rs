use crate::il2cpp::types::Il2CppImage;

pub mod Query;
pub mod Connection;

pub fn init(image: *const Il2CppImage) {
    Query::init(image);
    Connection::init(image);
}