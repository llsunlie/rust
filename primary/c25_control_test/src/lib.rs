fn prints_and_returns_10(a: i32) -> i32 {
    println!("got value: {}", a);
    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

mod tests {
    use crate::{prints_and_returns_10, add_two};

    /* 通过则不显示其中的 pringln 内容 */
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    /* cargo test with name */
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}