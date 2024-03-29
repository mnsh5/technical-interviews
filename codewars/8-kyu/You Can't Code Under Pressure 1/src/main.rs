// You Can't Code Under Pressure #1

// Code as fast as you can! You need to double the integer and return it.

// Link: Code as fast as you can! You need to double the integer and return it.

fn double_integer(n: i32) -> i32 {
    let num = n * 2;
    num as i32
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::double_integer;

    #[test]
    fn sample_tests() {
        assert_eq!(double_integer(1), 2);
        assert_eq!(double_integer(5), 10);
        assert_eq!(double_integer(10), 20);
        assert_eq!(double_integer(20), 40);
        assert_eq!(double_integer(40), 80);
    }
}