//! Diagnostic dependency substitution, NOT a reachable production fault.
#![allow(dead_code)]
mod export {
    pub mod images {
        fn compress_zlib(_: &[u8]) -> Result<Vec<u8>, &'static str> {
            Err("diagnostic compressor failure")
        }
        #[path = "/repos/Antigravity/typst-crystalline/03_infra/src/export/images/icc.rs"]
        mod icc;
        pub fn forced_stream() -> Vec<u8> {
            icc::build_icc_profile_stream(icc::srgb_icc_profile_bytes())
        }
    }
}
fn main() {
    std::fs::write(std::env::args().nth(1).unwrap(), export::images::forced_stream()).unwrap();
}
