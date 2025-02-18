extern crate elements_miniscript as miniscript;
extern crate regex;

use std::str::FromStr;

use miniscript::{Miniscript, NoExt, Segwitv0};

fn do_test(data: &[u8]) {
    let s = String::from_utf8_lossy(data);
    if let Ok(desc) = Miniscript::<String, Segwitv0, NoExt>::from_str(&s) {
        let str2 = desc.to_string();
        let desc2 = Miniscript::<String, Segwitv0, NoExt>::from_str(&str2).unwrap();
        assert_eq!(desc, desc2);
    }
}

fn main() {
    loop {
        honggfuzz::fuzz!(|data| {
            do_test(data);
        });
    }
}

#[cfg(test)]
mod tests {
    use miniscript::elements::hex::FromHex;

    #[test]
    fn duplicate_crash() {
        let hex = Vec::<u8>::from_hex("00").unwrap();
        super::do_test(&hex);
    }
}
