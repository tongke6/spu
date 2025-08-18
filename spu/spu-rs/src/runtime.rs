use crate::{FieldType, ProtocolKind};
use libspu_sys::ffi;

pub struct RuntimeConfig {
    pub inner: cxx::UniquePtr<ffi::RuntimeConfig>,
}

impl RuntimeConfig {
    pub fn new(protocol: ProtocolKind, field: FieldType, fxp_fraction_bits: i64) -> Self {
        Self {
            inner: ffi::new_runtime_config(protocol.into(), field.into(), fxp_fraction_bits),
        }
    }
}

pub struct SpuRuntime {
    _runtime: cxx::UniquePtr<ffi::SPUContext>,
}

impl SpuRuntime {
    pub fn new(config: &RuntimeConfig, rank: usize) -> Self {
        // This is a simplified setup for a 2-party in-memory link.
        let mut desc = ffi::new_context_desc();
        ffi::add_party(&mut desc, "party0", "localhost:9530");
        ffi::add_party(&mut desc, "party1", "localhost:9531");

        let lctx = ffi::create_mem_link_context(&desc, rank);

        let runtime = ffi::new_spu_context(&config.inner, lctx);
        Self { _runtime: runtime }
    }
}
