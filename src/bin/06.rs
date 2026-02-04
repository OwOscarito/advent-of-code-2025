advent_of_code::solution!(6);

enum Operation {
    Mult,
    Add,
}

pub fn part_one(input: &str) -> Option<i64> {
    let lines: Vec<&str> = input.lines().collect();
    let lines_count = lines.len();

    let operation_line: Vec<Operation> = lines[lines_count - 1]
        .split_whitespace()
        .map(|s| match s {
            "*" => Operation::Mult,
            "+" => Operation::Add,
            &_ => panic!("Unexpected Symbol: {s:?}"),
        })
        .collect();
    let problem_count = operation_line.len();
    let mut problem_result: Vec<i64> = lines[0]
        .split_whitespace()
        .map(|s| match s.parse::<i64>() {
            Ok(n) => n,
            Err(e) => panic!("Couldn't parse number: {e:?}"),
        })
        .collect();

    for line_idx in 1..lines_count - 1 {
        let line_numbers: Vec<i64> = lines[line_idx]
            .split_whitespace()
            .map(|s| match s.parse::<i64>() {
                Ok(n) => n,
                Err(e) => panic!("Couldn't parse number: {e:?}"),
            })
            .collect();
        for problem_idx in 0..problem_count {
            problem_result[problem_idx] = match operation_line[problem_idx] {
                Operation::Add => problem_result[problem_idx] + line_numbers[problem_idx],
                Operation::Mult => problem_result[problem_idx] * line_numbers[problem_idx],
            };
        }
    }
    println!("{problem_result:?}");

    let mut grand_total = 0;
    for result in problem_result {
        grand_total += result;
    }
    Some(grand_total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let lines_count = lines.len();

    let operation_line_chars = lines[lines_count - 1].chars();
    let problem_info: Vec<(Operation, usize)> = Vec::new();

    let idx = 0;
    for c in operation_line_chars {
        if c == '*' {
            problem_info.push((Operation::Mult, 0));
        } else if c == '+' {
            problem_info.push((Operation::Add, 0));
        } else {
            let last_idx = problem_info.len() - 1;
            problem_info[last_idx].1 += 1;
        }
    }

    let problem_count = problem_info.len();
    let problem_numbers: Vec<Vec<u64>> = Vec::with_capacity(problem_count);

    for i in 0..lines_count - 1 {
        let mut line_chars = lines[i].chars();
        for p in problem_info {
            let problem_len = p.1;
        }
    }

    let mut grand_total = 0;
    for result in problem_result {
        grand_total += result;
    }
    Some(grand_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
