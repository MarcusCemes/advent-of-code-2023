advent_of_code::solution!(2);

const MAX_SET: Set = Set {
    red: 12,
    green: 13,
    blue: 14,
};

#[derive(Debug, Default)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse_line)
            .filter(|game| game.is_valid())
            .map(|game| game.id)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse_line)
            .map(|game| game.required_cubes())
            .map(|set| set.power())
            .sum(),
    )
}

fn parse_line(line: &str) -> Game {
    let (left, right) = line.split_once(':').unwrap();
    let id = left[5..].parse().unwrap();

    let set_it = right.split(';').map(|set_str| {
        set_str
            .split(',')
            .map(|term| term.trim())
            .map(|term| term.split_once(' ').unwrap())
            .fold(Set::default(), |mut set, (count_str, colour)| {
                let count = count_str.parse().unwrap();

                match colour {
                    "red" => set.red = count,
                    "green" => set.green = count,
                    "blue" => set.blue = count,
                    _ => panic!("Unknown colour: {}", colour),
                }

                set
            })
    });

    Game {
        id,
        sets: set_it.collect(),
    }
}

impl Game {
    fn is_valid(&self) -> bool {
        self.sets.iter().all(|set| set.is_valid())
    }

    fn required_cubes(&self) -> Set {
        self.sets.iter().fold(Set::default(), |mut set, other| {
            set.red = set.red.max(other.red);
            set.green = set.green.max(other.green);
            set.blue = set.blue.max(other.blue);
            set
        })
    }
}

impl Set {
    fn is_valid(&self) -> bool {
        self.red <= MAX_SET.red && self.green <= MAX_SET.green && self.blue <= MAX_SET.blue
    }

    fn power(&self) -> u32 {
        [self.red, self.green, self.blue].iter().product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
