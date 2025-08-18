use spu_rs::io::IoClient;
use spu_rs::runtime::{RuntimeConfig, SpuRuntime};
use spu_rs::{FieldType, ProtocolKind};
use libspu_sys::ffi;

fn main() {
    let config = RuntimeConfig::new(ProtocolKind::Semi2k, FieldType::Fm64, 18);
    let runtime = SpuRuntime::new(&config, 0);
    let io = IoClient::new(2, &config);

    let data = vec![1i64, 2, 3, 4];

    let shares = io.make_shares(&data, ffi::Visibility::VIS_SECRET, 0);

    println!("Successfully created {} shares.", shares.len());
}
