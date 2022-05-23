#![no_main]
use libfuzzer_sys::fuzz_target;

use mail_parser::parsers::message::MessageStream;
use mail_parser::parsers::fields::raw::parse_raw;

fuzz_target!(|data: &[u8]| {
    let mut stream = MessageStream::new(&data);
    let _ = parse_raw(&mut stream);
});