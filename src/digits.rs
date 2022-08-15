#[allow(dead_code)]
fn digits(num: f64) -> impl Iterator<Item = u32> {
    num.to_string()
        .chars()
        .map(|i| i.to_digit(10).unwrap_or_else(|| 0))
        .collect::<Vec<u32>>()
        .into_iter()
}

pub fn get_string_digits(num: &str) -> Vec<u32> {
    num.chars()
        .enumerate()
        .map(|(i, d)| {
            d.to_digit(10).unwrap_or_else(|| {
                println!("suspicious i: {}", i);
                panic!();
            })
        })
        .collect()
}

#[allow(dead_code)]
fn get_digits(n: usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();

    fn digits_rec(num: usize, res: &mut Vec<usize>) {
        if num > 10 {
            let num = num / 10;
            digits_rec(num, res);
        }
        res.push(num % 10);
    }
    digits_rec(n, &mut result);
    result
}

// https://stackoverflow.com/questions/41536479/how-do-i-split-an-integer-into-individual-digits
pub fn digits_iter(mut num: usize) -> impl Iterator<Item = usize> {
    let mut divisor = 1;
    while num >= divisor * 10 {
        divisor *= 10;
    }

    std::iter::from_fn(move || {
        if divisor == 0 {
            None
        } else {
            let v = num / divisor;
            num %= divisor;
            divisor /= 10;
            Some(v)
        }
    })
}

// not used, left here for fun
pub struct Digits {
    n: usize,
    divisor: usize,
}

impl Digits {
    pub fn new(num: usize) -> Digits {
        let mut divisor = 1;
        while num >= divisor * 10 {
            divisor *= 10;
        }
        Digits { n: num, divisor }
    }
}

impl Iterator for Digits {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 0 {
            None
        } else {
            let v = self.n / self.divisor;
            self.n %= self.divisor;
            self.divisor /= 10;
            Some(v)
        }
    }
}
