#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
extern crate bytecount;

#[cfg(test)]
mod tests {
    use bytecount::{count, naive_count};
    use quickcheck::QuickCheck;

    fn prop(xs: Vec<u8>, x: u8) -> bool {
        let a = count(&xs, x);
        let b = naive_count(&xs, x);
        if a != b {
            //println!("{:?} vs. {:?}", a, b);
        }
        a == b
    }

    #[test]
    fn quickcheck() {
        let n = 100_000;
        QuickCheck::new()
            .tests(n)
            .max_tests(n * 100)
            .quickcheck(prop as fn(Vec<u8>, u8) -> bool);
    }

    #[test]
    fn issue_found_by_quickcheck() {
        let xs = &[0, 0, 0, 0, 0, 0, 0, 0, 33, 32, 0, 0, 0, 0, 0, 0];
        let x = 33;
        assert_eq!(count(xs, x), naive_count(xs, x));
    }
}
