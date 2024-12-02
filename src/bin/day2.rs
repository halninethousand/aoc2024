fn main() {
    let input: Vec<Vec<u32>> = include_str!("../../data/day2.txt").lines()
        .map(|line| line.split_whitespace().map(|n| n.parse::<u32>().unwrap())
            .collect())
        .collect(); 

    let mut safe: u32 = 0;
    
    for line in &input {
        if is_increasing(line) || is_decreasing(line) {
            safe += 1;
        }
    }

    println!("Part 1 safe: {}", safe);

    safe = 0;

    'outer: for line in &input {
        if is_increasing(line) || is_decreasing(line) {
            safe += 1;
            continue 'outer;
        }

        for x in 0..line.len() {
            let mut li = line.clone();
            li.remove(x);
            if is_increasing(&li) || is_decreasing(&li) {
                safe += 1;
                continue 'outer;
            }
        }
    }

    println!("Part 2 safe: {}", safe);

}


fn is_decreasing(line: &Vec<u32>) -> bool {
    line.windows(2).all(|pair| {
        let diff = pair[0] as i32 - pair[1] as i32;
        diff == 1 || diff == 2 || diff == 3
    })
}

fn is_increasing(line: &Vec<u32>) -> bool {
    line.windows(2).all(|pair| {
        
        let diff = pair[1] as i32 - pair[0] as i32;
        diff == 1 || diff == 2 || diff == 3
    })
}
