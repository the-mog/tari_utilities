#![no_main]
use libfuzzer_sys::fuzz_target;

use tari_utilities::hex;

fuzz_target!(|data: &[u8]| {
    hex::to_hex_multiple(&[data.to_vec()]);
});
