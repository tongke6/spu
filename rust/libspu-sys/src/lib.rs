#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("libspu/spu.h");

        type ProtocolKind;
        type FieldType;
    }
}
