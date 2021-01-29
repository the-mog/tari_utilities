#![no_main]
use libfuzzer_sys::fuzz_target;

use tari_utilities::fixed_set::FixedSet;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data){
        if let Ok(n) = s.parse::<usize>(){
            let mut fs = FixedSet::<usize>::new(n);
            let _set = fs.set_item(n, n);
            let _search = fs.search(&n);
            let _clear = fs.clear_item(n);
        }
    }
});
