use std::vec;

use crate::{
    custom_error::AocError,
    utils::{CharMap, Point},
};

// Finds coordinates of all empty rows and columns
fn find_expansions(map: &CharMap) -> (Vec<usize>, Vec<usize>) {
    let mut empty_columns = vec![];
    for x in 0..map.width() {
        let mut empty = true;
        for y in 0..map.height() {
            // TODO: make point generic over type
            if *map.cell(x as i64, y as i64) != '.' {
                empty = false;
                break;
            }
        }
        if empty {
            empty_columns.push(x);
        }
    }

    let mut empty_rows = vec![];
    for (y, line) in map.lines().enumerate() {
        if line.iter().all(|c| *c == '.') {
            empty_rows.push(y);
        }
    }

    return (empty_columns, empty_rows);
}

#[tracing::instrument]
pub fn process(input: &str, expansion_factor: usize) -> miette::Result<String, AocError> {
    let map = CharMap::from_str(input, '.');
    let galaxies = map.find_all('#');

    let (empty_cols, empty_rows) = find_expansions(&map);
    let mut expanded_galaxies = vec![];

    for galaxy in galaxies {
        let empty_to_left = empty_cols.iter().filter(|x| **x < galaxy.x as usize);
        let empty_to_top = empty_rows.iter().filter(|y| **y < galaxy.y as usize);

        let expansion_size_x = empty_to_left.clone().count() * (expansion_factor - 1);
        let expansion_size_y = empty_to_top.clone().count() * (expansion_factor - 1);

        let expanded_x = galaxy.x as usize + expansion_size_x;
        let expanded_y = galaxy.y as usize + expansion_size_y;

        println!(
            "Galaxy at {:?} expands to {:?}",
            galaxy,
            Point::new(expanded_x as i64, expanded_y as i64)
        );
        println!(
            "Empty to left: {:?}, empty to top: {:?}",
            empty_to_left.collect::<Vec<_>>(),
            empty_to_top.collect::<Vec<_>>()
        );

        expanded_galaxies.push(Point::new(expanded_x as i64, expanded_y as i64));
    }

    let mut sum = 0;
    for i in 0..expanded_galaxies.len() {
        let src = expanded_galaxies[i];
        for j in i + 1..expanded_galaxies.len() {
            let dst = expanded_galaxies[j];
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
    fn test_process_10x() -> miette::Result<()> {
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
        assert_eq!("1030", process(input, 10)?);
        Ok(())
    }

    #[test]
    fn test_process_100x() -> miette::Result<()> {
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
        assert_eq!("8410", process(input, 100)?);
        Ok(())
    }
}
