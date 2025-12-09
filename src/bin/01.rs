advent_of_code::solution!(1);

pub fn solution(input: &str) -> (i16, i16, i16) {
    input
        .lines()
        .filter_map(|value| {
            if value.starts_with("L") {
                value[1..].parse::<i16>().ok().map(|n| -n)
            } else {
                value[1..].parse::<i16>().ok()
            }
        })
        .fold((50, 0i16, 0i16), |previous, current| {
            let calculation = previous.0 + current;
            let result = ((calculation % 100) + 100) % 100;
            (
                result,

                if result == 0 {
                    previous.1 + 1
                } else {
                    previous.1
                },

                if calculation <= -100 || calculation >= 100 {
                    let wraps = ((calculation - (calculation % 100)) / 100).abs() + if previous.0 == 0 {-1} else {0};
                    println!("first(+{}): previous: {:?}, current: {}, calculation: {}", wraps, previous, current, calculation);
                    previous.2 + wraps
                } else if calculation == 0 {
                    println!("  second(+1): previous: {:?}, current: {}, calculation: {}", previous, current, calculation);
                    previous.2 + 1
                } else {
                    println!("      third(): previous: {:?}, current: {}, calculation: {}", previous, current, calculation);
                    previous.2
                },
            )
        })
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solution(input).1 as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solution(input).2 as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
