pub mod Enum;
pub mod Byte;

pub fn init() {
    get_assembly_image_or_return!(image, "mscorlib.dll");

    Enum::init(image);
    Byte::init(image);
}
