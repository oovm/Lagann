//! 网易云 ncm 格式

use hex_literal::hex;

const CORE_KEY: &[u8; 16] = &hex!("687A4852416D736F356B496E62617857");
const META_KEY: &[u8; 16] = &hex!("2331346C6A6B5F215C5D2630553C2728");
const HEAD_KEY: &[u8; 8] = &hex!("4354454E4644414D");

#[test]
fn test() {
    use crate::utils::u8_array_print;
    u8_array_print(CORE_KEY);
    u8_array_print(META_KEY);
    u8_array_print(HEAD_KEY);
}
