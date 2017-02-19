extern "C" {
    fn basiccount(buffer: *const u8, size: usize) -> usize;
    fn memchrcount(buffer: *const u8, size: usize) -> usize;
    fn swarcount(buffer: *const u8, size: usize) -> usize;
    fn avxcount(buffer: *const u8, size: usize) -> usize;
    fn avxcountuu(buffer: *const u8, size: usize) -> usize;
    fn avxcountu(buffer: *const u8, size: usize) -> usize;
}

pub fn newlinecount_basic(buffer: &[u8]) -> usize {
    unsafe { basiccount(buffer.as_ptr(), buffer.len()) }
}

pub fn newlinecount_memchr(buffer: &[u8]) -> usize {
    unsafe { memchrcount(buffer.as_ptr(), buffer.len()) }
}

pub fn newlinecount_swar(buffer: &[u8]) -> usize {
    unsafe { swarcount(buffer.as_ptr(), buffer.len()) }
}

pub fn newlinecount_avx(buffer: &[u8]) -> usize {
    unsafe { avxcount(buffer.as_ptr(), buffer.len()) }
}

pub fn newlinecount_avxuu(buffer: &[u8]) -> usize {
    unsafe { avxcountuu(buffer.as_ptr(), buffer.len()) }
}

pub fn newlinecount_avxu(buffer: &[u8]) -> usize {
    unsafe { avxcountu(buffer.as_ptr(), buffer.len()) }
}

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
        let n = 1_000_000;
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
