use regex::Regex;

fn main() {
    let input: Vec<&str> = include_str!("../../data/day3.txt").lines().collect();

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let mut muls_sum = 0;

    for line in input {
        for cap in re.find_iter(line) {
            println!("Matched: {}", cap.as_str());
            let mut string = cap.as_str();
            string = remove_last_char(&string[4..]);
            println!("{}", string);
            let nums: Vec<i32> = string.split(',').filter_map(|n| n.parse::<i32>().ok()).collect();

            println!("nums: {:?}", nums);
            muls_sum += nums[0] * nums[1];
        }
    }

    println!("Part 1 sum: {}", muls_sum);
}

fn remove_last_char(s: &str) -> &str {
    if s.is_empty() {
        s
    } else {
        &s[..s.len() - s.chars().last().unwrap().len_utf8()]
    }
}
