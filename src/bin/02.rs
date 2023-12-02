advent_of_code::solution!(2);

/* == Constants ==  */

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

/* == Definitions == */

#[derive(Debug, Default)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game<'a> {
    id: u32,
    set_str: &'a str, // Reference avoids heap allocating a Vec<Set>
}

/* == Solutions == */

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(Game::parse_str)
        .filter(Game::is_valid)
        .map(|game| game.id)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(Game::parse_str)
        .map(|game| game.required_cubes())
        .map(|set| set.cube_power())
        .sum();

    Some(result)
}

impl Game<'_> {
    fn parse_str(line: &str) -> Game {
        let (id_str, set_str) = line.split_once(':').unwrap();
        let id = id_str[5..].parse().unwrap();
        Game { id, set_str }
    }

    fn iter_sets(&self) -> impl Iterator<Item = Set> + '_ {
        self.set_str.split(';').map(Set::parse_str)
    }

    fn is_valid(&self) -> bool {
        self.iter_sets().all(|set| set.is_valid())
    }

    fn required_cubes(&self) -> Set {
        self.iter_sets().fold(Set::default(), |mut set, other| {
            set.red = set.red.max(other.red);
            set.green = set.green.max(other.green);
            set.blue = set.blue.max(other.blue);
            set
        })
    }
}

impl Set {
    fn parse_str(str: &str) -> Self {
        str.split(',')
            .map(|term| term.trim().split_once(' ').unwrap())
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
    }

    fn is_valid(&self) -> bool {
        self.red <= MAX_RED && self.green <= MAX_GREEN && self.blue <= MAX_BLUE
    }

    fn cube_power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

/* == Tests == */

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
