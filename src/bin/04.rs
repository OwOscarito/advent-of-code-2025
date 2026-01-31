use std::fmt::Display;

advent_of_code::solution!(4);

fn print_matrix<T: Display>(matrix: Vec<Vec<T>>) {
  for row in matrix {
    for elem in row{
      print!("{elem}")
    }
    println!()
  }
}

pub fn part_one(input: &str) -> Option<u32> {
  let char_matrix: Vec<Vec<char>> = input.lines()
    .map(|line| line.chars().collect())
    .collect();
  let height = char_matrix.len();
  let width = char_matrix[0].len();
  let mut number_matrix: Vec<Vec<u8>> = vec![vec![0u8; width]; height];

  for y in 0..height {
    for x in 0..width {
      if char_matrix[y][x] == '@' {
        let before_row_exist = y != 0;
        let after_row_exist = y != height - 1;

        let before_col_exist = x != 0;
        let after_col_exist = x != width - 1;

        if before_row_exist && before_col_exist {
          number_matrix[y - 1][x - 1] += 1;
        }
        if before_row_exist {
          number_matrix[y - 1][x] += 1;
        }
        if before_row_exist && after_col_exist {
          number_matrix[y - 1][x + 1] += 1;
        }

        if before_col_exist {
          number_matrix[y][x - 1] += 1;
        }
        if after_col_exist {
          number_matrix[y][x + 1] += 1;
        }

        if after_row_exist && before_col_exist {
          number_matrix[y + 1][x - 1] += 1;
        }
        if after_row_exist {
          number_matrix[y + 1][x] += 1;
        }
        if after_row_exist && after_col_exist {
          number_matrix[y + 1][x + 1] += 1;
        }
      }
    }
  }

  let mut accessible_rolls = 0;
  for y in 0..height {
    for x in 0..width {
      if char_matrix[y][x] == '@' && number_matrix[y][x] < 4 {
        accessible_rolls += 1;
      }
    }
  }
  Some(accessible_rolls)
}

pub fn part_two(input: &str) -> Option<u64> {
  let mut removed_total = 0;

  let mut char_matrix: Vec<Vec<char>> = input.lines()
    .map(|line| line.chars().collect())
    .collect();
  let height = char_matrix.len();
  let width = char_matrix[0].len();

  loop {
    let mut number_matrix: Vec<Vec<i16>> = vec![vec![0; width]; height];
    for y in 0..height {
      for x in 0..width {
        if char_matrix[y][x] == '@' {
          let before_row_exist = y != 0;
          let after_row_exist = y != height - 1;

          let before_col_exist = x != 0;
          let after_col_exist = x != width - 1;

          if before_row_exist && before_col_exist {
            number_matrix[y - 1][x - 1] += 1;
          }
          if before_row_exist {
            number_matrix[y - 1][x] += 1;
          }
          if before_row_exist && after_col_exist {
            number_matrix[y - 1][x + 1] += 1;
          }

          if before_col_exist {
            number_matrix[y][x - 1] += 1;
          }
          if after_col_exist {
            number_matrix[y][x + 1] += 1;
          }

          if after_row_exist && before_col_exist {
            number_matrix[y + 1][x - 1] += 1;
          }
          if after_row_exist {
            number_matrix[y + 1][x] += 1;
          }
          if after_row_exist && after_col_exist {
            number_matrix[y + 1][x + 1] += 1;
          }
        }
      }
    }

    let mut removed = 0;
    for y in 0..height {
      for x in 0..width {
        if char_matrix[y][x] == '@' && number_matrix[y][x] < 4 {
          removed += 1;
          char_matrix[y][x] = '.';
        }
      }
    }
    if removed == 0 {
      break
    }
    removed_total += removed;
    println!("{removed}");
    print_matrix(number_matrix);
  }


  print_matrix(char_matrix);

  Some(removed_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
