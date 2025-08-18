use crate::runtime::RuntimeConfig;
use libspu_sys::ffi;
use std::ffi::c_void;

pub struct IoClient {
    pub inner: cxx::UniquePtr<ffi::IoClient>,
}

impl IoClient {
    pub fn new(world_size: usize, config: &RuntimeConfig) -> Self {
        Self {
            inner: ffi::new_io_client(world_size, &config.inner),
        }
    }

    pub fn make_shares<T: Pod>(&self, data: &[T], vtype: ffi::Visibility, owner_rank: i32) -> Vec<cxx::UniquePtr<ffi::Value>> {
        let pt_type = T::pt_type();
        let shape = vec![data.len() as i64];
        let strides = vec![1];
        let ptr = data.as_ptr() as *const u8;

        unsafe {
            let bv = ffi::new_pt_buffer_view(ptr, pt_type, &shape, &strides);
            ffi::make_shares(&self.inner, &bv, vtype, owner_rank)
        }
    }
}

pub trait Pod {
    fn pt_type() -> ffi::PtType;
}

impl Pod for i64 {
    fn pt_type() -> ffi::PtType {
        ffi::PtType::PT_I64
    }
}

// Add more impls for other primitive types here...
