#![no_main]
use libfuzzer_sys::fuzz_target;

use tari_utilities::convert;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data){
        if let Ok(n) = s.parse::<u32>(){
            let v: Vec<u32> = vec![n,n,n,n,n,n];
            let _ = convert::try_convert_all(v);
        }
    }
});
