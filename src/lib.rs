/// implementing euclidean algorithm for finding the greatest common divisor
/// of integers a and b.
pub fn gcd(a: i64, b: i64) -> i64 {
    let mut pre = a;
    let mut las = b;

    while las != 0 {
        let tmp = pre % las;

        pre = las;
        las = tmp;
    }

    pre
}

pub fn lcm(a: i64, b: i64) -> i64 {
    b / gcd(a, b) * a
}

pub fn eular_cal_coefficient(a: i64, b: i64) -> (i64, (i64, i64)) {
    let mut pre = a;
    let mut las = b;
    let mut pre_co: (i64, i64) = (1, 0);
    let mut las_co: (i64, i64) = (0, 1);

    while las != 0 {
        let tmp: i64 = pre % las;
        let constant: i64 = pre / las;
        let tmp_pre = -1 * constant * las_co.0 + pre_co.0;
        let tmp_las = -1 * constant * las_co.1 + pre_co.1;

        pre_co = las_co;
        las_co = (tmp_pre, tmp_las);

        pre = las;
        las = tmp;
    }
    (pre, pre_co)
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Colorize;
    use lazy_static::lazy_static;

    struct TestCases {
        a: i64,
        b: i64,
        expected: i64,
    }

    lazy_static! {
        static ref TEST_CASES: Vec<TestCases> = vec![
            TestCases {
                a: 120,
                b: 180,
                expected: 60
            },
            TestCases {
                a: 180,
                b: 120,
                expected: 60
            },
            TestCases {
                a: 120,
                b: 0,
                expected: 120
            },
            TestCases {
                a: 75614,
                b: 12345,
                expected: 1
            },
            TestCases {
                a: 1234512,
                b: 65756,
                expected: 4
            },
            TestCases {
                a: 13412,
                b: 77831,
                expected: 1,
            },
            TestCases {
                a: 56429164134,
                b: 7641826,
                expected: 2,
            },
        ];
    }

    #[test]
    fn test_gcd() {
        fn test(a: i64, b: i64, expected: i64) {
            if gcd(a, b) != expected {
                println!(
                    "{}",
                    format!("gcd({}, {}) = {}: expected {expected}", a, b, gcd(a, b)).red()
                );
                panic!();
            }
            println!("gcd({}, {}) = {}", a, b, gcd(a, b));
        }

        for i in TEST_CASES.iter() {
            test(i.a, i.b, i.expected);
        }
    }

    #[test]
    fn test_eular_cal_coefficient() {
        fn test(a: i64, b: i64, expected: i64) {
            let res = eular_cal_coefficient(a, b);

            // test gcd
            if res.0 != expected {
                println!(
                    "{}",
                    format!("gcd({}, {}) = {}: expected {expected}", a, b, gcd(a, b)).red()
                );
                panic!();
            }
            println!("gcd({}, {}) = {}", a, b, gcd(a, b));

            // test sum
            if a * res.1 .0 + b * res.1 .1 != res.0 {
                println!("calculated: {} * {} + {} * {} = {}", a, res.1 .0, b, res.1 .1, a * res.1 .0 + b * res.1 .1);
                println!("{}", format!("expected: {}", expected).red());
                panic!();
            } else {
                println!("{} * {} + {} * {} = {}", a, res.1 .0, b, res.1 .1, a * res.1 .0 + b * res.1 .1);
            }
        }

        for i in TEST_CASES.iter() {
            test(i.a, i.b, i.expected);
        }
    }
}
