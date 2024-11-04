use rug::{Rational, Complete};

pub trait MakeFloat {
    fn mf(self) -> Rational;
}

impl MakeFloat for u32 {
    fn mf(self) -> Rational {
        Rational::from((self, 1))
    }
}

impl MakeFloat for f32 {
    fn mf(self) -> Rational {
        Rational::from(((self * 1_000_000.0) as u32, 1_000_000))
    }
}

impl MakeFloat for &str {
    fn mf(self) -> Rational {
        match self.parse::<f32>() {
            Ok(f) => f.mf(),
            Err(_) => Rational::from((0, 1))
        }
    }
}

pub trait MakePrintable {
    fn mp(self) -> String;
}

impl MakePrintable for Rational {
    fn mp(self) -> String {
        kdiv_str(self)
    }
}

fn ksqrt_get_largest_possible(n: &Rational, p: &Rational) -> (Rational, Rational) {
    let mut f = 0.0.mf();

    while f.clone() < *n {
        let lhs = p + f.clone() + 1.0.mf();
        let rhs = f.clone() + 1.0.mf();
        if lhs * rhs > *n {
            break;
        }

        f = f + 1.0.mf();
    }

    let sq = (p + f.clone()) * f.clone();

    (f, n - sq)
}

pub fn ksqrt(mut n: Rational) -> Rational {
    let mut factor = 1.0.mf();

    let mut result: Rational = 0.0.mf();

    let mut prepend: Rational = 0.0.mf();

    let mut i = 0;
    while i < 3000 {
        let (sqrt, rest) = ksqrt_get_largest_possible(&n, &prepend);

        result = result + (&sqrt * &factor).complete();

        prepend = (sqrt * 2.0.mf() + prepend) * 10.0.mf();

        n = rest * 100.0.mf();

        factor = factor * 0.1.mf();

        i += 1;
    }

    result
}

pub fn kdiv_str(n: Rational) -> String {
    let num = n.numer();
    let den = n.denom();

    let mut s = String::new();

    let integer_part = (num / den).complete();
    s.push_str(&integer_part.to_string());

    let mut rem = (num % den).complete();

    if rem == 0 {
        return s;
    }

    s.push_str(".");

    let mut i = 0;
    while i < 3000 {
        rem *= 10;
        let dec_digit = (&rem / den).complete();
        s.push_str(&dec_digit.to_string());
        rem %= den;

        if rem == 0 {
            break;
        }

        i += 1;
    }

    s
}
