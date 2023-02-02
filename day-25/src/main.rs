use std::fs;

enum SNAFUDigit {
    Zero,
    One,
    Two,
    Minus,
    DoubleMinus,
}

impl SNAFUDigit {
    fn from_char(c: char) -> SNAFUDigit {
        match c {
            '0' => SNAFUDigit::Zero,
            '1' => SNAFUDigit::One,
            '2' => SNAFUDigit::Two,
            '-' => SNAFUDigit::Minus,
            '=' => SNAFUDigit::DoubleMinus,
            _ => panic!("Invalid SNAFU digit"),
        }
    }
}

type SNAFUNumber = Vec<SNAFUDigit>;

trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for SNAFUNumber {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for digit in self.iter().rev() {
            result.push(match digit {
                SNAFUDigit::Zero => '0',
                SNAFUDigit::One => '1',
                SNAFUDigit::Two => '2',
                SNAFUDigit::Minus => '-',
                SNAFUDigit::DoubleMinus => '=',
            });
        }
        result
    }
}

trait Toi64 {
    fn to_i64(&self) -> i64;
}

impl Toi64 for SNAFUNumber {
    fn to_i64(&self) -> i64 {
        let mut result = 0;
        let mut multiplier = 1;
        for digit in self.iter() {
            result += match digit {
                SNAFUDigit::Zero => 0,
                SNAFUDigit::One => 1,
                SNAFUDigit::Two => 2,
                SNAFUDigit::Minus => -1,
                SNAFUDigit::DoubleMinus => -2,
            } * multiplier;
            multiplier *= 5;
        }
        result
    }
}

trait Fromi64 {
    fn from_i64(num: i64) -> SNAFUNumber;
}

impl Fromi64 for SNAFUNumber {
    fn from_i64(num: i64) -> SNAFUNumber {
        let mut result = SNAFUNumber::new();
        let mut remaining = num;
        while remaining != 0 {
            let (normal_div, rem) = (remaining / 5, remaining % 5);
            let (div, cur) = match rem {
                0 => (normal_div, SNAFUDigit::Zero),
                1 => (normal_div, SNAFUDigit::One),
                2 => (normal_div, SNAFUDigit::Two),
                3 => (normal_div + 1, SNAFUDigit::DoubleMinus),
                4 => (normal_div + 1, SNAFUDigit::Minus),
                _ => panic!("Invalid SNAFU digit"),
            };
            result.push(cur);
            remaining = div;
        }
        return result;
    }
}

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents
        .split("\n")
        .map(|line| line.trim())
        .collect::<Vec<&str>>();

    let snafu_numbers = lines
        .iter()
        .map(|line| {
            line.chars()
                .rev()
                .map(|c| SNAFUDigit::from_char(c))
                .collect::<SNAFUNumber>()
        })
        .collect::<Vec<SNAFUNumber>>();
    let sum = snafu_numbers.iter().map(|num| num.to_i64()).sum::<i64>();
    let sum_snafu = SNAFUNumber::from_i64(sum);
    println!("SNAFU sum: {}", sum_snafu.to_string());
}
