// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let static_ref = v.leak();
    let (left_half, right_half) = static_ref.split_at(static_ref.len() / 2);

    let left_handle = thread::spawn(|| left_half.iter().sum::<i32>());
    let right_handle = thread::spawn(|| right_half.iter().sum::<i32>());

    match left_handle.join() {
        Err(_) => panic!("Something bad happened"),
        Ok(left_res) => match right_handle.join() {
            Err(_) => panic!("Something bad happened"),
            Ok(right_res) => left_res + right_res,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
