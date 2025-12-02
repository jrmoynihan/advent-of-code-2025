#![feature(int_roundings)]
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut zeros = 0;
    let mut dial: i32 = 50;
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        // Split the line at the first character
        let (left, right) = line.split_at(1);
        let right = right.parse::<i32>().unwrap();

        // Process the left and right parts of the line
        if left == "L" {
            // println!("turning left {:?} clicks", right);
            dial = (dial - right) % 100;
            // println!("dial: {}", dial);
        } else if left == "R" {
            // println!("turning right {:?} clicks", right);
            dial = (dial + right) % 100;
            // println!("dial: {}", dial);
        }
        // println!("dial: {}", dial);
        if dial == 0 {
            zeros += 1;
        }
    }
    Some(zeros)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut zeros: u64 = 0;
    let mut dial: i64 = 50;
    let mut lines = input.lines();
    let mut temp = 0;

    while let Some(line) = lines.next() {
        // Split the line at the first character
        let (left, right) = line.split_at(1);
        let mut right = right.parse::<i64>().unwrap();

        // Process the left and right parts of the line
        if left == "L" {
            temp = dial;
            dial = (dial - right) % 100;
            while right > 0 {
                temp -= 1;
                right -= 1;
                println!("temp: {}, right: {}", temp, right);
            }
            println!("temp: {}", temp);
            zeros += (temp.abs() % 100) as u64;
            println!("zeros: {}", zeros);
            // println!("turning left {:?} clicks", right);
            // println!("dial: {}", dial);
        } else if left == "R" {
            temp = dial;
            dial = (dial + right) % 100;
            while right > 0 {
                temp += 1;
                right -= 1;
                println!("temp: {}, right: {}", temp, right);
                // if temp == 100 {
                //     zeros += 1;
                //     println!("crossed hundred, zeros: {}", zeros);
                // }
            }
            println!("temp: {}", temp);
            zeros += (temp.abs() % 100) as u64;
            println!("zeros: {}", zeros);
            // println!("turning right {:?} clicks", right);
            // println!("dial: {}", dial);
        }
        if dial == 0 {
            zeros += 1;
        }
        println!("dial: {}", dial);
        println!("zeroes: {}", zeros);
        println!("------------")
    }
    Some(zeros)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        println!("Result: {:?}", result);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        println!("Result: {:?}", result);
        assert_eq!(result, Some(6));
    }
}
