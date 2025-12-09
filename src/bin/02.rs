advent_of_code::solution!(2);

struct Range {
    min: u64,
    max: u64,
}

impl Range {
    fn create(value: &str) -> Option<Range> {
        let split = value.split("-").collect::<Vec<&str>>();
        let min = split[0].parse().ok();
        let max = split[1].parse().ok();

        match (min, max) {
            (Some(a), Some(b)) => Some(Range { min: a, max: b }),
            _ => None,
        }
    }

    fn get_invalid_ids(self) -> Vec<u64> {
        let mut ids = vec![];
        for i in self.min..self.max + 1 {
            let amount = i.ilog10();
            if amount % 2 != 1 {
                continue;
            }

            let string_number = i.to_string();
            let split_number = string_number.split_at(((amount + 1) / 2) as usize);
            if split_number.0 == split_number.1 {
                ids.push(i)
            }
        }

        ids
    }

    fn get_invalid_ids_pattern(self) -> Vec<u64> {
        (self.min..self.max + 1)
            .filter_map(|i| {
                Some(i)
            }).collect::<Vec<u64>>()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .split(",")
        .filter_map(Range::create)
        .map(Range::get_invalid_ids)
        .flatten()
        .sum::<u64>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input
        .split(",")
        .filter_map(Range::create)
        .map(Range::get_invalid_ids_pattern)
        .flatten()
        .sum::<u64>();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
