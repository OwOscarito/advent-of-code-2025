advent_of_code::solution!(1);

enum Direction {
    Right,
    Left,
}

fn parse_rotation(rotation: &str) -> Result<(i32, Direction), &'static str> {
  let len = rotation.chars().count();
  if len < 2 {
    return Err("Line too short {rotation_info}");
  }

  let (direction_str, distance_str) = rotation.split_at(1);

  let distance =  match distance_str.parse::<i32>() {
    Ok(distance) => distance,
    Err(_) => return Err("Could not parse {distance_str} to i32"),
  };

  let direction = match direction_str {
    "R" => Direction::Right,
    "L" => Direction::Left,
    _ => return Err("Last character must be 'R' or 'L'."),
  };

  Ok((distance, direction))
}

pub fn part_one(input: &str) -> Option<i32> {
  let mut password: i32  = 0;
  let mut position: i32 = 50;

  for rotation in input.lines() {
    let (distance, direction) = parse_rotation(rotation).expect("Failed Parsing");

    position = match direction {
      Direction::Right => (position + distance).rem_euclid(100),
      Direction::Left => (position - distance).rem_euclid(100)
    };

    if position == 0 {
      password += 1
    }
  }
  Some(password)
}

// Method0x434C49434B
pub fn part_two(input: &str) -> Option<i32> {
  let mut password: i32  = 0;
  let mut position: i32 = 50;

  for rotation in input.lines() {
    let (distance, direction) = parse_rotation(rotation).expect("Failed Parsing");

    let mut next_position = match direction {
      Direction::Right => position + distance,
      Direction::Left => position - distance
    };
    next_position = next_position.rem_euclid(100);

    password += distance / 100;
    let remainder = match direction {
      Direction::Right => distance % 100,
      Direction::Left => -(distance % 100),
    };

    if position != 0 && (position + remainder <= 0 || position + remainder >= 100) {
      password += 1;
    }
    position = next_position;
  }
  Some(password)
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
