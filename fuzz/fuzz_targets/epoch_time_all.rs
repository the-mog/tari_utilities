#![no_main]
use libfuzzer_sys::fuzz_target;

use tari_utilities::epoch_time::EpochTime;

fuzz_target!(|data: EpochTime| {
    let et = EpochTime::now();
    let _ = et.checked_sub(data);
});
