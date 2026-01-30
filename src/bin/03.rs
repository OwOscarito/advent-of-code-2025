advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
  let mut joltage: u32 = 0;

  for bank in input.lines() {
    let bank_chars = bank.chars().collect::<Vec<char>>();
    let mut max: u32 = 0;
    let mut max_idx: usize = 0;

    for i in 0..bank_chars.len()-1 {
      let bank_digit = bank_chars[i].to_digit(10)?;
      if bank_digit > max {
        max = bank_digit;
        max_idx = i;
      }
    }

    let mut second_max = 0;
    for i in max_idx + 1..bank_chars.len() {
      let bank_digit = bank_chars[i].to_digit(10)?;

      if bank_digit > second_max {
        second_max = bank_digit;
      }
    }
    println!("{max_idx}: {max} | {second_max}");

    joltage += (max*10) + second_max;
  }
  Some(joltage)
}

pub fn part_two(input: &str) -> Option<u64> {
  let mut joltage: u64 = 0;

  for bank in input.lines() {
    let base: u32 = 10;
    let bank_digits = bank.chars().map(|c| {c.to_digit(base).expect("Failed to parse number")}).collect::<Vec<u32>>();
    let bank_len = bank_digits.len();
    let mut batteries: Vec<u32> = vec![0; 12];
    let mut batteries_idx: Vec<usize> = vec![0; 12];

    // Find max
    for bank_idx in 0..bank_len - 12 {
      let bank_digit = bank_digits[bank_idx];
      if bank_digit > batteries[0] {
        batteries[0] = bank_digit;
        batteries_idx[0] = bank_idx
      }
    }

    for batt_idx in 1..12 {
      let start_idx = batteries_idx[batt_idx - 1] + 1;
      let end_idx = bank_len - 11 + batt_idx;

      if bank_len - start_idx < 12 - batt_idx {
        println!("{bank_len} - {start_idx} < 12 - {batt_idx}");
        for i in 0..13-batt_idx {
          batteries[batt_idx+i] = bank_digits[start_idx+i];
          batteries_idx[batt_idx+i] = start_idx+i;
        }
        break;
      }

      for bank_idx in start_idx..end_idx {
        if bank_digits[bank_idx] > batteries[batt_idx] {
          batteries[batt_idx] =  bank_digits[bank_idx];
          batteries_idx[batt_idx] = bank_idx
        }
      }
    }

    println!("{:?}", batteries);
    println!("{:?}", batteries_idx);
    println!();

    let bank_joltage: u64 = batteries.iter().fold(0, |acc, elem| acc * 10 + (*elem as u64));

    joltage += bank_joltage;

  }
Some(joltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
