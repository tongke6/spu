pub mod runtime;
pub mod io;

use libspu_sys::ffi;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolKind {
    Invalid,
    Ref2k,
    Semi2k,
    Aby3,
    Cheetah,
    Securenn,
}

impl From<ffi::ProtocolKind> for ProtocolKind {
    fn from(kind: ffi::ProtocolKind) -> Self {
        match kind {
            ffi::ProtocolKind::REF2K => ProtocolKind::Ref2k,
            ffi::ProtocolKind::SEMI2K => ProtocolKind::Semi2k,
            ffi::ProtocolKind::ABY3 => ProtocolKind::Aby3,
            ffi::ProtocolKind::CHEETAH => ProtocolKind::Cheetah,
            ffi::ProtocolKind::SECURENN => ProtocolKind::Securenn,
            _ => ProtocolKind::Invalid,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    Invalid,
    Fm32,
    Fm64,
    Fm128,
}

impl From<ffi::FieldType> for FieldType {
    fn from(field: ffi::FieldType) -> Self {
        match field {
            ffi::FieldType::FM32 => FieldType::Fm32,
            ffi::FieldType::FM64 => FieldType::Fm64,
            ffi::FieldType::FM128 => FieldType::Fm128,
            _ => FieldType::Invalid,
        }
    }
}
