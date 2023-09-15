use crate::try_outs::functions::Feline;
pub mod try_outs;


fn main() {

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    dbg!(v);
}