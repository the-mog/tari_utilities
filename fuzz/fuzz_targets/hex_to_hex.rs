#![no_main]
use libfuzzer_sys::fuzz_target;

use tari_utilities::hex;

fuzz_target!(|data: &[u8]| {
    let _ = hex::to_hex(data);
});
