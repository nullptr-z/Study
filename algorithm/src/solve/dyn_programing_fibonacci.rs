fn recursion_fibonacci(n: usize) -> usize {
    if n < 2 {
        return n as usize;
    }
    return recursion_fibonacci(n - 1) + recursion_fibonacci(n - 2);
}

fn dyn_programing_fibonacci(n: usize) -> u128 {
    let mut fib_vec: Vec<u128> = Vec::with_capacity(n);

    fib_vec.push(1);
    fib_vec.push(1);
    for i in 2..n {
        let num: u128 = (fib_vec[i - 1] + fib_vec[i - 2]).try_into().unwrap();
        fib_vec.push(num);
    }
    fib_vec.pop().unwrap()
    // return dyn_programing_fibonacci(n - 1, vec) + dyn_programing_fibonacci(n - 2, vec);
}

pub fn unique_paths(m: i32, n: i32) -> i32 {
    let row = (m) as usize;
    let col = (n) as usize;
    let mut cache = Vec::with_capacity(row);
    for _ in 0..row {
        cache.push(vec![1; col]);
    }
    for r in 1..row {
        for l in 1..(col) {
            let sum = cache[r][l - 1] + cache[r - 1][l];
            cache[r][l] = sum;
        }
    }

    cache[row - 1][col - 1]
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::solve::dyn_programing_fibonacci::unique_paths;

    use super::{dyn_programing_fibonacci, recursion_fibonacci};

    #[test]
    fn crete_recursion() {
        let n = 10;

        let now = Instant::now();
        let untie_recursion = recursion_fibonacci(1);
        let nanos = now.elapsed().as_nanos();
        let micros = now.elapsed().as_micros();
        let millis = now.elapsed().as_millis();

        let now = Instant::now();
        let untie_dyn_programing = dyn_programing_fibonacci(n);
        let nanos = now.elapsed().as_nanos();
        let micros = now.elapsed().as_micros();
        let millis = now.elapsed().as_millis();

        let now = Instant::now();
        let untie = unique_paths(3, 7);
        let nanos = now.elapsed().as_nanos();
        let micros = now.elapsed().as_micros();
        let millis = now.elapsed().as_millis();

        // 55
        assert_eq!(untie, 28);
        // assert_eq!(untie_recursion, 55);
        // assert_eq!(untie_dyn_programing, 55);
    }
}
