advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i64> {
  let mut invalid_ids = 0;

  let ranges = input.split(",");

  for range in ranges {
    let mut parts = range.trim().split("-");
    let start_str = parts.next().expect(format!("Couldn't obtain first string").as_str());
    let end_str = parts.next().expect(format!("Couldn't obtain first string").as_str());

    let start = start_str.parse::<i64>().expect(format!("Couldn't parse {start_str}").as_str());
    let end = end_str.parse::<i64>().expect(format!("Couldn't parse {end_str}").as_str());

    for id in start..end+1 {
      let id_str: String = id.to_string();
      let id_str_len = id_str.chars().count();
      if id_str_len % 2 == 1 {
        continue;
      }
      let (first_half, second_half) = id_str.split_at(id_str_len / 2);

      if first_half == second_half {
        invalid_ids += id;
      }
    }
  }
  return Some(invalid_ids);
}

pub fn part_two(input: &str) -> Option<i128> {
  let mut invalid_ids = 0;

  let ranges = input.split(",");

  for range in ranges {
    let mut parts = range.trim().split("-");
    let start_str = parts.next().expect(format!("Couldn't obtain first string").as_str());
    let end_str = parts.next().expect(format!("Couldn't obtain first string").as_str());

    let start = start_str.parse::<i128>().expect(format!("Couldn't parse {start_str}").as_str());
    let end = end_str.parse::<i128>().expect(format!("Couldn't parse {end_str}").as_str());

    for id in start..end+1 {
      let id_str: String = id.to_string();
      let id_chars: Vec<char> = id_str.chars().collect();
      let id_len = id_chars.len();

      for chunk_size in 1..(id_len/2)+1 {
        if id_len % chunk_size != 0 {
          continue;
        }

        let mut chunks = id_chars.chunks(chunk_size);

        let mut equal = true;
        let first_chunk = chunks.next().unwrap();
        for chunk in chunks {
          if chunk != first_chunk {
            equal = false;
            break;
          }
        }

        if equal {
          invalid_ids += id;
          break;
        }
      }
    }
  }
  return Some(invalid_ids);
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
