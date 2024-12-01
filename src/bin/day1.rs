fn main() {
    let input: Vec<&str> = include_str!("../../data/day1.txt").lines().collect();
    
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in &input {
        let split: Vec<i32> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();
        left.push(split[0]);
        right.push(split[1]);

    }

    left.sort();
    right.sort();
        
    let zipped: Vec<_> = left.iter().zip(right.iter()).collect();
    let sum: Vec<i32> = zipped.iter().map(|(a, b)| {let diff = *a - *b; diff.abs()}).collect();

    println!("Part 1 sum: {}", sum.iter().sum::<i32>());
    
    let mut similarity = 0;

    for item in left {
        let count = right.iter().filter(|&&x| x == item).count();
        similarity += count as i32 * item;
        println!("{item} appears {count} times in right");

    }

    println!("Part 2 similarity score: {}", similarity);

}
