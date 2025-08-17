use crate::{FieldType, ProtocolKind};
use libspu_sys::ffi;

pub struct RuntimeConfig {
    pub protocol: ProtocolKind,
    pub field: FieldType,
    pub fxp_fraction_bits: i64,
}

impl RuntimeConfig {
    fn to_ffi(&self) -> ffi::RuntimeConfig {
        // This is not a complete implementation.
        // I will need to implement the full conversion later.
        ffi::RuntimeConfig::new(
            self.protocol.into(),
            self.field.into(),
            self.fxp_fraction_bits,
        )
    }
}

pub struct SpuRuntime {
    _runtime: cxx::UniquePtr<ffi::SPUContext>,
}

impl SpuRuntime {
    pub fn new(config: &RuntimeConfig) -> Self {
        // This is not a complete implementation.
        // I will need to implement the full conversion later.
        let ffi_config = config.to_ffi();
        let lctx = unimplemented!(); // I need a link context here.
        let runtime = ffi::SPUContext::new(ffi_config, lctx);
        Self { _runtime: runtime }
    }
}
