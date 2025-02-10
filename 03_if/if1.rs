fn bigger(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", bigger(10, 8)); // Should print 10
    println!("{}", bigger(42, 32)); // Should print 42
    println!("{}", bigger(42, 42)); // Should print 42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
