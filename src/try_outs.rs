pub mod functions;
pub mod collections;

pub fn plus_one(op: Option<i32>) -> Option<i32>{
    match op {
        None => None,
        Some(t) => Some(t+1),
    }
}

