use crate::{custom_error::AocError, utils::CharMap};

// Doubles the size of each empty row and column in the map
fn expand_map(map: &CharMap) -> CharMap {
    let mut empty_columns = vec![];
    for x in 0..map.width() {
        let mut empty = true;
        for y in 0..map.height() {
            if *map.cell(x as i64, y as i64) != '.' {
                empty = false;
                break;
            }
        }
        if empty {
            empty_columns.push(x);
        }
    }

    let mut new_map: Vec<Vec<char>> = vec![];
    for (y, line) in map.lines().enumerate() {
        let mut new_line = vec![];

        let mut empty = true;
        for (x, cell) in line.iter().enumerate() {
            if *cell != '.' {
                empty = false;
            }
            if empty_columns.contains(&x) {
                new_line.push('.');
                new_line.push('.');
            } else {
                new_line.push(*cell);
            }
        }

        if empty {
            new_map.push(new_line.clone());
        }
        new_map.push(new_line);
    }

    let new_width = new_map.iter().map(|line| line.len()).max().unwrap();
    let mut new_char_map = CharMap::from_dimensions(new_width, new_map.len(), '.');
    new_char_map.copy_from_vec(&new_map);

    return new_char_map;
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let map = CharMap::from_str(input, '.');
    let map = expand_map(&map);
    map.print();

    let galaxies = map.find_all('#');
    let mut sum = 0;
    for i in 0..galaxies.len() {
        let src = galaxies[i];
        for j in i + 1..galaxies.len() {
            let dst = galaxies[j];
            let distance = src.manhattan_distance(&dst);
            println!("Distance from {:?} to {:?} is {}", src, dst, distance);
            sum += distance;
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "...#......
                     .......#..
                     #.........
                     ..........
                     ......#...
                     .#........
                     .........#
                     ..........
                     .......#..
                     #...#.....";
        assert_eq!("374", process(input)?);
        Ok(())
    }
}
