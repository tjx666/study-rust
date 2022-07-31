#![allow(dead_code)]
#![allow(unused_variables)]

use std::ops::Add;

///
/// ```
/// assert_eq!(tests::add(0.1, 0.1), 0.2);
/// ```
pub fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    return a + b;
}

// get_first 是一个内部函数
// rust 是允许对私有函数测试的
fn get_first(v: &Vec<i32>) -> &i32 {
    println!("Test private function");
    &v[0]
}

#[cfg(test)]
mod tests {
    use std::io::Error as IOError;

    use crate::{add, get_first};

    #[test]
    fn add_two_integer() {
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(1, 1), 2);
        assert_eq!(add(1, 2), 3)
    }

    #[test]
    fn add_two_float() {
        assert_eq!(add(1.2, 2.1), 3.3);
    }

    #[test]
    #[should_panic]
    fn get_negative_index_element() {
        println!("stdout xxx");
        get_first(&vec![]);
    }

    #[test]
    fn test_return_result() -> Result<(), IOError> {
        // Err(IOError::new(std::io::ErrorKind::Other, ""))
        Ok(())
    }

    

    #[test]
    #[ignore]
    fn expensive_test() {
        println!("need long time to calculate...");
    }
}
