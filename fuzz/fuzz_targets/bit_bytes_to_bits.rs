#![no_main]
use libfuzzer_sys::fuzz_target;

use tari_utilities::bit;

fuzz_target!(|data: &[u8]| {
    let _ = bit::bytes_to_bits(data);

});
