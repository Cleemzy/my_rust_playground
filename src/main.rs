use crate::try_outs::functions;
pub mod try_outs;
// pub mod try_outs;

fn main() {

    let o = Some(6);

    let r = functions::plus_one(o);

    println!("{:?}", r);


}