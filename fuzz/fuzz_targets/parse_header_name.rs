#![no_main]
use libfuzzer_sys::fuzz_target;

use mail_parser::parsers::header::parse_header_name;

fuzz_target!(|data: &[u8]| {
    let _ = parse_header_name(&data);
});