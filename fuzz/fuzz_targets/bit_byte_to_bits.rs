#![no_main]
use libfuzzer_sys::fuzz_target;

use tari_utilities::bit;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data){
        if let Ok(n) = s.parse::<u8>(){
            let _ = bit::byte_to_bits(n);
        }
    }
});
