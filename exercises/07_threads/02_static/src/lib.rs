// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread;

pub fn sum(slice: &'static [i32]) -> i32 {
    let split_point = (slice.len() / 2) as usize;
    // let left_half = &slice[0..split_point];
    // let right_half = &slice[split_point..];

    let left_handle = thread::spawn(move || slice[..split_point].iter().sum::<i32>());
    let right_handle = thread::spawn(move || slice[split_point..].iter().sum::<i32>());

    let mut res = match left_handle.join() {
        Ok(ret_value) => ret_value,
        Err(_) => panic!("Something bad happened"),
    };

    res = match right_handle.join() {
        Ok(ret_value) => ret_value + res,
        Err(_) => panic!("Something bad happened"),
    };

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
