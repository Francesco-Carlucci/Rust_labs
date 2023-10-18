
// manual checks
fn count(minefield: &[&str], x: usize, y: usize) -> u8 {
    let mut count = 0;

    let i0 = if x > 0 { x - 1 } else { 0 };
    let j0 = if y > 0 { y - 1 } else { 0 };
    let i1 = if x < minefield[0].len() - 1 { x + 1 } else { minefield[0].len() -1 };
    let ji = if y < minefield.len() - 1 { y + 1 } else { minefield.len() - 1 };

    for i in i0..=i1 {
        for j in j0..=ji {
            if i == x && j == y {
                continue;
            }
            if minefield[j].as_bytes()[i] == '*' as u8 {
                count += 1;
            }
        }
    }
    count
}

// more idiomatic
fn count2(minefield: &[&str], x: usize, y: usize) -> u8 {
    let mut count = 0;
    for yi in y.saturating_sub(1)..=(y+1).min(minefield.len()-1) {
        for xj in x.saturating_sub(1)..=(x+1).min(minefield[yi].len()-1) {
            if yi == y &&  x == xj {
                continue;
            }
            if minefield[yi].as_bytes()[xj] == '*' as u8 {
                count += 1;
            }
        }
    }
    count 
} 

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut res = Vec::new();

    for (y, line) in minefield.iter().enumerate() {
        let mut s = String::new();
        for (x, c) in line.as_bytes().iter().enumerate() {
            if *c == '*' as u8 {
                s.push('*');
            } else {
                //println!("{}", count2(minefield, x, y));
                match count2(minefield, x, y) {
                    0 => s.push(' '),
                    i @ 0..=8 => s.push_str(&format!("{}", i)),
                    _ => panic!("Invalid count"),
                }
            }
        }
        res.push(s);
    }
    res
}
