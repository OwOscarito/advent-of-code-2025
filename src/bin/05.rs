advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut fresh_count: u64 = 0;
    let mut lines = input.lines();

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    loop {
        let line = match lines.next() {
            Some("") => break,
            Some(l) => l,
            None => break,
        };
        let mut parts = line.split('-');
        let start: u64 = parts.next()?.parse().ok()?;
        let end: u64 = parts.next()?.parse().ok()?;
        ranges.push((start, end));
    }

    for id in lines {
        for (range_start, range_end) in &ranges {
            let id_num: u64 = id.parse().ok()?;
            if id_num >= *range_start && id_num <= *range_end {
                fresh_count += 1;
                break;
            }
        }
    }
    Some(fresh_count)
}

struct RangedList {
    ranges: Vec<(u64, u64)>,
}
impl RangedList {
    fn new() -> Self {
        RangedList { ranges: Vec::new() }
    }
    fn insert_range(&mut self, start: u64, end: u64) {
        if self.len() == 0 {
            self.ranges.push((start, end));
        }
        let (start_pos, start_inside) = self.search_in(start, 0, self.ranges.len());
        let (end_pos, end_inside) = self.search_in(end, start_pos, self.ranges.len());

        if start_pos == end_pos {
            if !start_inside && !end_inside {
                self.ranges.insert(start_pos, (start, end));
            } else if !start_inside && end_inside {
                self.ranges[start_pos].0 = start;
            } else if start_inside && !end_inside {
                self.ranges[start_pos].1 = end;
            }
        } else {
            if !start_inside && !end_inside {
                self.ranges.insert(start_pos, (start, end));
            } else if !start_inside && end_inside {
                self.ranges[start_pos].0 = start;
            } else if start_inside && !end_inside {
                self.ranges[start_pos].1 = end;
            }
        }
    }
    fn search_in(&mut self, num: u64, start_pos: usize, end_pos: usize) -> (usize, bool) {
        let mut left = start_pos;
        let mut right = end_pos;

        let mut last_idx = 0;
        while left <= right {
            let mid = right / 2;
            let mid_start = self.ranges[mid].0;
            let mid_end = self.ranges[mid].1;

            if mid_start <= num && num <= mid_end {
                return (mid, true);
            } else if num < mid_start {
                right = mid - 1;
            } else if mid_end < num {
                left = mid + 1;
            }
            last_idx = mid;
        }
        return (last_idx, false);
    }
    fn count(&mut self) -> u64 {
        let mut count = 0;
        for (start, end) in &self.ranges {
            count += end - start + 1;
        }
        count
    }
    fn len(&mut self) -> usize {
        self.ranges.len()
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut fresh_count: u64 = 0;
    let mut lines = input.lines();

    let mut fresh_ingredients = RangedList::new();
    loop {
        let line = match lines.next() {
            Some("") => break,
            Some(l) => l,
            None => break,
        };
        let mut parts = line.split('-');
        let start: u64 = parts.next()?.parse().ok()?;
        let end: u64 = parts.next()?.parse().ok()?;

        fresh_ingredients.insert_range(start, end);
    }

    fresh_count += fresh_ingredients.count();

    Some(fresh_count)
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
        assert_eq!(result, Some(14));
    }
}
