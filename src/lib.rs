use num_traits::Float;

use std::collections::HashMap;

pub enum MulDiv {
    Mul,
    Div,
}

pub struct MultMult<T: Float> {
    points: HashMap<usize, isize>,
    value: Option<T>,
    primes: HashMap<usize, isize>,
    max_number: usize,
}

impl<T: Float> MultMult<T> {
    pub fn new() -> Self {
        let points = HashMap::new();
        let value = None;
        let mut primes = HashMap::new();
        primes.insert(2, 0);
        let max_number = 2;

        Self { points, value, primes, max_number }
    }

    fn muldiv(&mut self, (start, end): (usize, usize), mode: MulDiv) {
        let (s, e) = match mode {
            MulDiv::Mul => (start, end+1),
            MulDiv::Div => (end+1, start),
        };
        let counter = self.points.entry(s).or_insert(0);
        *counter += 1;
        let counter = self.points.entry(e).or_insert(0);
        *counter -= 1;

        self.value = None;
    }

    fn expand_primes(&mut self, n: usize) {
        if n <= self.max_number {
            return;
        }
        let mut p = self.primes.keys().map(|&k| k).collect::<Vec<usize>>();
        p.sort();
        for number in self.max_number+1..=n {
            let mut flg = true;
            for &pi in p.iter() {
                if number % pi == 0 {
                    flg = false;
                    break;
                }
                if pi * pi >= number {
                    break;
                }
            }
            if flg {
                p.push(number);
                self.primes.insert(number, 0);
            }
        }
        self.max_number = n;
    }

    fn factrize(&mut self, number: usize, times: isize) {
        if times == 0 {
            return;
        }
        let mut n = number;
        let keys: Vec<usize> = self.primes.keys().map(|&k| k).collect();
        for &pi in keys.iter() {
            loop {
                if n % pi == 0 {
                    n /= pi;
                    let counter = self.primes.entry(pi).or_insert(0);
                    *counter += times;
                } else {
                    break;
                }
            }
        }
        if n != 1 {
            panic!();
        }
    }

    pub fn value(&mut self) -> T {
        match self.value {
            Some(v) => { return v }
            None => {}
        };

        let mut keys: Vec<usize> = self.points.keys().map(|&k| k).collect();
        keys.sort();
        let n_keys = keys.len();
        let max_key = keys[n_keys - 1];
        let max_number = self.max_number;
        if max_key > max_number {
            self.expand_primes(max_key);
        }
        let (mut count, mut number): (isize, usize) = (0, 0);
        for &ki in keys.iter() {
            if count == 0 {
                number = ki;
            } else {
                while number < ki {
                    self.factrize(number, count);
                    number += 1;
                }
            }
            count += self.points[&ki];
            self.factrize(number, count);
            number += 1;
        }

        let ans = self.mul_primes();
        self.value = Some(ans);
        self.points = HashMap::new();

        ans
    }

    fn mul_primes(&self) -> T {
        let mut nume = 1;
        let mut deli = 1;
        for (&k, &v) in self.primes.iter() {
            if v > 0 {
                nume *= k.pow(v as u32);
            } else {
                deli *= k.pow((-v) as u32);
            }
        }

        T::from(nume).unwrap() / T::from(deli).unwrap()
    }

    pub fn mul_fact(&mut self, number: usize) {
        self.muldiv(perm(number, number), MulDiv::Mul);
    }

    pub fn div_fact(&mut self, number: usize) {
        self.muldiv(perm(number, number), MulDiv::Div);
    }

    pub fn mul_perm(&mut self, left: usize, right: usize) {
        self.muldiv(perm(left, right), MulDiv::Mul);
    }

    pub fn div_perm(&mut self, left: usize, right: usize) {
        self.muldiv(perm(left, right), MulDiv::Div);
    }

    pub fn mul_comb(&mut self, left: usize, right: usize) {
        self.muldiv(perm(left, right), MulDiv::Mul);
        self.muldiv(perm(right, right), MulDiv::Div);
    }

    pub fn div_comb(&mut self, left: usize, right: usize) {
        self.muldiv(perm(left, right), MulDiv::Div);
        self.muldiv(perm(right, right), MulDiv::Mul);
    }
}

fn perm(left: usize, right: usize) -> (usize, usize) {
    if left < right {
        panic!()
    }
    if left == 0 || right == 0 {
        (1, 1)
    } else {
        (left - right + 1, left)
    }
}