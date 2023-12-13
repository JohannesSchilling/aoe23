use std::io::stdin;
use std::collections::HashMap;

fn read_grid() -> Vec<Vec<u8>> {
    stdin()
        .lines()
        .map(|s| s.expect("read line").into_bytes())
        .collect()
}

fn sign_around(
    x0: usize,
    x1: usize,
    y: usize,
    grid: &Vec<Vec<u8>>,
) -> Option<(usize, usize)> {
    let search_row = |row: &[u8]| -> Option<usize> {
        let min = x0.checked_sub(1).unwrap_or(0);
        let max = std::cmp::min(x1 + 1, row.len()-1);
        for i in min..=max {
            if !row[i].is_ascii_digit() && row[i] != b'.' {
                // some other sign, yay
                return Some(i);
            }
        }
        None
    };

    if y > 0 {
        // row before 
        if let Some(x) = search_row(&grid[y-1]) {
            return Some((x, y-1))
        }
    }

    if y < grid.len()-1 {
        // row after
        if let Some(x) = search_row(&grid[y+1]) {
            return Some((x, y+1));
        }
    }

    // char before number digits
    if x0 > 0 {
        let b = grid[y][x0 - 1];
        if !b.is_ascii_digit() && b != b'.' {
            return Some((x0-1, y));
        }
    }

    // char after all digits
    if x1 + 1 < grid[y].len() {
        let b = grid[y][x1 + 1];
        if !b.is_ascii_digit() && b != b'.' {
            return Some((x1 + 1, y));
        }
    }

    None
}


fn follow_digits(start: usize, line: &[u8]) -> Option<(usize, usize)> {
    if !line[start].is_ascii_digit() {
        return None;
    }

    let mut sum = 0;
    let mut i = start;
    while i < line.len() && line[i].is_ascii_digit() {
        sum *= 10;
        sum += line[i] as usize - '0' as usize;
        i += 1;
    }

    Some((i-1, sum))
}

fn main() {
    let grid = read_grid();
    let mut total1 = 0;
    // if not set: you're the first to touch that gear; if set: multiply yourself to it
    let mut gears = HashMap::new();
    let mut total2 = 0;

    for y in 0..grid.len() {
        let mut x = 0;
        while x < grid[y].len() {
            if let Some((end, n)) = follow_digits(x, &grid[y]) {
                // found a number, see if there's a sign around, then add and follow around
                if let Some((x, y)) = sign_around(x, end, y, &grid) {
                    total1 += n;
                    if grid[y][x] ==  b'*' {
                        // gear
                        // multiply by self or insert self
                        // negative self to later know which ones are not real
                        // gears and just one factor
                        if let Some(other) = gears.get(&(x, y)) {
                            total2 += n * other;
                            gears.remove(&(x, y)); // to be safe
                        } else {
                            gears.insert((x, y), n);
                        }
                    }
                }
                x = end+1;
            } else {
                // nothing found, just go next digit
                x += 1;
            }
        }
    }

    println!("day3/1 sum = {total1}");
    println!("day3/2 sum = {total2}");
}
