use crate::{custom_error::AocError, utils::CharMap};

// Gets a map depicting rocks (round as O and square as #) and updates the map
// to calculate how it would look like if the platform was tilted north and all
// the round rocks rolled until stopping.
fn tilt(map: &mut CharMap) {
    for col in 0..map.width() {
        let mut stop = 0; // place where a rock would stop if it were to roll north
        for row in 0..map.height() {
            let c = *map.cell(col as i64, row as i64);
            if c == '#' {
                stop = row + 1;
            }

            if c == 'O' {
                map.set_cell(col, row, '.');
                map.set_cell(col, stop, c);
                stop = stop + 1;
            }
        }
    }
}

fn load(map: &CharMap) -> usize {
    let mut total_load = 0;
    for col in 0..map.width() {
        for row in 0..map.height() {
            if *map.cell(col as i64, row as i64) == 'O' {
                let load = map.height() - row;
                total_load = total_load + load;
            }
        }
    }
    return total_load;
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut map = CharMap::from_str(input, '@');
    map.print();
    tilt(&mut map);
    let total_load = load(&map);
    return Ok(total_load.to_string());
}

#[cfg(test)]
mod tests {
    use crate::utils::CharMap;

    use super::*;

    #[test]
    fn test_tilt() {
        let input = "O....#....
                     O.OO#....#
                     .....##...
                     OO.#O....O
                     .O.....O#.
                     O.#..O.#.#
                     ..O..#O..O
                     .......O..
                     #....###..
                     #OO..#....";

        let mut map = CharMap::from_iter(input.lines().map(|s| s.trim()), '@');
        tilt(&mut map);

        let expected = "OOOO.#.O..
                        OO..#....#
                        OO..O##..O
                        O..#.OO...
                        ........#.
                        ..#....#.#
                        ..O..#.O.O
                        ..O.......
                        #....###..
                        #....#....";
        let expected_map = CharMap::from_iter(expected.lines().map(|s| s.trim()), '@');

        if map == expected_map {
            return;
        }

        println!("Actual:");
        map.print();

        println!("Expected:");
        expected_map.print();

        for row in 0..map.height() {
            for col in 0..map.width() {
                let real = map.cell(col as i64, row as i64);
                let expected = expected_map.cell(col as i64, row as i64);
                if real != expected {
                    println!("The maps are different as {}x{}:", col, row);
                    println!(" - real: {}", real);
                    println!(" - expected: {}", expected);
                }
            }
        }

        assert_eq!(expected_map, map);

        map.print();
    }

    #[test]
    fn test_load() {
        let input = "OOOO.#.O..
                     OO..#....#
                     OO..O##..O
                     O..#.OO...
                     ........#.
                     ..#....#.#
                     ..O..#.O.O
                     ..O.......
                     #....###..
                     #....#....";
        let map = CharMap::from_iter(input.lines().map(|s| s.trim()), '@');
        assert_eq!(136, load(&map));
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
