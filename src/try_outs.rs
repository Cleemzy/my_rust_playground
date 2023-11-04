pub mod functions;
pub mod collections;
pub mod generics;
pub mod independents;

pub fn plus_one(op: Option<i32>) -> Option<i32>{
    match op {
        None => None,
        Some(t) => Some(t+1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {

        assert_eq!(Some(4), plus_one(Some(3)));
    }
}
