advent_of_code::solution!(1);

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

const DIAL_MAX: i64 = 100;
const DIAL_MIN: i64 = 0;

impl Direction {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }
    pub fn spin(&self, dial: i64, clicks: i64) -> (i64, i64) {
        let new = match self {
            Direction::Left => dial - clicks,
            Direction::Right => dial + clicks,
        };
        let mut revolutions = (new / DIAL_MAX).abs();
        if dial != DIAL_MIN && new <= DIAL_MIN {
            revolutions += 1;
        }
        (new.rem_euclid(DIAL_MAX), revolutions)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut zeros = 0;
    let mut dial: i64 = 50;
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        // Split the line at the first character
        let (direction, clicks) = line.split_at(1);
        let clicks = clicks.parse::<i64>().unwrap();

        // Process the left and right parts of the line
        if let Some(direction) = Direction::from_char(direction.chars().next()?) {
            let (new, _) = direction.spin(dial, clicks);
            dial = new;
        }
        if dial == DIAL_MIN {
            zeros += 1;
        }
    }
    Some(zeros)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut zeros: u64 = 0;
    let mut dial: i64 = 50;
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        // Split the line at the first character
        let (direction, clicks) = line.split_at(1);
        let clicks = clicks.parse::<i64>().unwrap();

        // Process the left and right parts of the line
        if let Some(direction) = Direction::from_char(direction.chars().next()?) {
            let (new, revolutions) = direction.spin(dial, clicks as i64);
            // println!(
            //     "direction: {:?}, turning {clicks} clicks ({revolutions} revolutions) from {dial} to {new}",
            //     direction
            // );
            zeros += revolutions as u64;
            dial = new;
        }
        // println!("dial: {dial}, zeros: {zeros}");
        // println!("------------")
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
