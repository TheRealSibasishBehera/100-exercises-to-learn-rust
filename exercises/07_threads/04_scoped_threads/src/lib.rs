// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::thread;
pub fn sum(v: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut ans2 = 0;

    let mid = v.len() / 2;

    // with scope the val is not dropped after a spawn is over and is mainterd over the period of
    // spawn
    thread::scope(|s| {
        s.spawn(|| {
            let first = &v[..v.len() / 2];
            ans += first.iter().sum::<i32>();
        });
        s.spawn(|| {
            let second = &v[v.len() / 2..];
            ans2 += second.iter().sum::<i32>();
        });
    });
    ans + ans2
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
