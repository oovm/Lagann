use itertools::Itertools;

pub fn u8_array_print(a: &[u8]) {
    let mut v = a.iter().map(|c| format!("{:#04X}", c));
    println!("[{}]", v.join(", "))
}
