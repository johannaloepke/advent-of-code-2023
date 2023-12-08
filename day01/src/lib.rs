use std::collections::HashMap;

pub fn process_part1(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        let nums: Vec<char> = line.chars().filter(|c: &char| c.is_numeric()).collect();

        sum += format!("{}{}", nums.first().unwrap(), nums.last().unwrap())
            .parse::<u32>()
            .unwrap();  
    }
    
    return sum.to_string();
}

pub fn process_part2(input: &str) -> String {
    let numbers_map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9")
    ]);

    let mut sum = 0;
    
    for line in input.lines() {
        let mut first_index = line.len();
        let mut last_index = 0;
        let (mut first_word, mut last_word) = ("", "");
        for num in numbers_map.keys() {
            let found = line.find(num);
            let rfound = line.rfind(num);
            if found.is_some() && found.unwrap() < first_index {
                first_word = num;
                first_index = found.unwrap();
            }
            if rfound.is_some() && rfound.unwrap() >= last_index {
                last_word = num;
                last_index = rfound.unwrap();
            }
        }
        let mut newline = String::from(line);
        let mut first_replace = String::from(line);
        if numbers_map.get(first_word).is_some() {
            first_replace = newline.replace(first_word, numbers_map.get(first_word).unwrap());
        }
        // hacky way to concat both replaced strings in case there was overlap, the first and last numbers will still be valid
        if numbers_map.get(last_word).is_some() {
            newline = format!("{}{}", first_replace, newline.replace(last_word, numbers_map.get(last_word).unwrap()));
        }
        
        let nums: Vec<char> = newline.chars().filter(|c: &char| c.is_numeric()).collect();

        sum += format!("{}{}", nums.first().unwrap(), nums.last().unwrap())
            .parse::<u32>()
            .unwrap();  
    }
    
    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = process_part1(input);
        assert_eq!(result, "142");
    }
    #[test]
    fn two_works() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        let result = process_part2(input);
        assert_eq!(result, "281");
    }
}
