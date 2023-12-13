use std::io::{BufRead, BufReader, stdin};


fn to_num(ds: &[char]) -> usize {
    if ds.is_empty() {
        // make the day2 sample work ;-)
        return 0;
    }

    let fst = ds[0] as usize - '0' as usize;
    let snd = ds[ds.len()-1] as usize - '0' as usize;

    10 * fst + snd
}

const NUMBERS: [&str; 10] = [ "ZERO", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ];
// XXX: i think this doesn't work, i found an example that ends with "..twone",
// and it might be required to return 1 here, while this parses left-to-right
// and would return 2.
fn _parse_named_nums(mut s: String) -> Vec<usize> {
    let mut digits = Vec::new();

    'outer: while !s.is_empty() {
        if s.as_bytes()[0].is_ascii_digit() {
            digits.push(s.as_bytes()[0] as usize - '0' as usize);
            s = s.split_off(1);
            continue 'outer;
        }
        // try to find the word it starts with
        for (num, word) in NUMBERS.iter().enumerate() {
            if s.starts_with(word) {
                digits.push(num);
                s = s.split_off(word.len());
                continue 'outer;
            }
        }

        // nothing found, try next char
        s = s.split_off(1);
    }

    digits
}

fn first_digit(s: &str) -> usize {
    if s.as_bytes()[0].is_ascii_digit() {
        return s.as_bytes()[0] as usize - '0' as usize;
    }

    for (num, word) in NUMBERS.iter().enumerate() {
        if s.starts_with(word) {
            return num;
        }
    }

    return first_digit(s.get(1..).unwrap());
}

fn last_digit(s: &str) -> usize {
    let last_byte = s.as_bytes()[s.as_bytes().len()-1];
    if last_byte.is_ascii_digit() {
        return last_byte as usize - '0' as usize;
    }

    for (num, word) in NUMBERS.iter().enumerate() {
        if s.ends_with(word) {
            return num;
        }
    }

    return last_digit(s.get(..s.len()-1).unwrap());
}

fn main() {
    let reader = BufReader::new(stdin());
    let mut total: usize = 0;
    let mut total2: usize = 0;

    for line in reader.lines() {
        let line = line.expect("io error");
        let digits = line.chars().filter(char::is_ascii_digit).collect::<Vec<_>>();
        total += to_num(&digits);

        let n = 10 * first_digit(&line) + last_digit(&line);
        dbg!(line, n);
        total2 += n
    }
    println!("day1/1 total = {total}");
    println!("day1/2 total = {total2}");
}
