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

fn spin_cycle(map: &CharMap) -> CharMap {
    let mut map = map.clone();
    for _ in 0..4 {
        tilt(&mut map);
        map = map.rotate_right();
    }
    return map;
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut map = CharMap::from_str_with_trim(input, '@');

    let mut seen = std::collections::HashMap::new();
    seen.insert(map.hash64(), 0);
    let mut cycle_start = 0;
    let mut cycle_len = 0;

    let max_cycles = 1000000000;
    for i in 1..=max_cycles {
        map = spin_cycle(&map);
        if seen.contains_key(&map.hash64()) {
            cycle_start = seen[&map.hash64()];
            cycle_len = i - cycle_start;
            println!(
                "Found a cycle after {} spins! Cycle start: {}",
                i, cycle_start
            );
            break;
        } else {
            seen.insert(map.hash64(), i);
        }
    }

    let left_cycles = (max_cycles - cycle_start) % cycle_len;
    println!(
        "Left cycles: {} ({} - {} % {})",
        left_cycles, max_cycles, cycle_start, cycle_len
    );
    for _ in 0..left_cycles {
        map = spin_cycle(&map);
    }

    let total_load = load(&map);
    return Ok(total_load.to_string());
}

#[cfg(test)]
mod tests {
    use core::panic;

    use crate::utils::CharMap;

    use super::*;

    fn assert_maps_eq(map1: &CharMap, map2: &CharMap) {
        if map1 == map2 {
            return;
        }

        let square_rocks1 = map1.count('#');
        let square_rocks2 = map2.count('#');
        if square_rocks1 != square_rocks2 {
            println!("The maps have different number of square rocks:");
            println!(" - real: {}", square_rocks1);
            println!(" - expected: {}", square_rocks2);
        }

        let round_rocks1 = map1.count('O');
        let round_rocks2 = map2.count('O');
        if round_rocks1 != round_rocks2 {
            println!("The maps have different number of round rocks:");
            println!(" - real: {}", round_rocks1);
            println!(" - expected: {}", round_rocks2);
        }

        println!("Actual:");
        map1.print();

        println!("Expected:");
        map2.print();

        for row in 0..map1.height() {
            for col in 0..map1.width() {
                let real = map1.cell(col as i64, row as i64);
                let expected = map2.cell(col as i64, row as i64);
                if real != expected {
                    println!("The maps are different as {}x{}:", col, row);
                    println!(" - real: {}", real);
                    println!(" - expected: {}", expected);
                }
            }
        }

        panic!("The maps are different!");
    }

    #[test]
    fn test_spin_cycle() {
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

        let map = CharMap::from_str_with_trim(input, '@');

        // After 1 cycle
        let map = spin_cycle(&map);
        let expected = ".....#....
                        ....#...O#
                        ...OO##...
                        .OO#......
                        .....OOO#.
                        .O#...O#.#
                        ....O#....
                        ......OOOO
                        #...O###..
                        #..OO#....";
        let expected_map = CharMap::from_str_with_trim(expected, '@');
        assert_maps_eq(&map, &expected_map);

        // After 2 cycles
        let map = spin_cycle(&map);
        let expected = ".....#....
                        ....#...O#
                        .....##...
                        ..O#......
                        .....OOO#.
                        .O#...O#.#
                        ....O#...O
                        .......OOO
                        #..OO###..
                        #.OOO#...O";
        let expected_map = CharMap::from_str_with_trim(expected, '@');
        assert_maps_eq(&map, &expected_map);

        // After 3 cycles
        let map = spin_cycle(&map);
        let expected = ".....#....
                        ....#...O#
                        .....##...
                        ..O#......
                        .....OOO#.
                        .O#...O#.#
                        ....O#...O
                        .......OOO
                        #...O###.O
                        #.OOO#...O";
        let expected_map = CharMap::from_str_with_trim(expected, '@');
        assert_maps_eq(&map, &expected_map);
    }

    #[test]
    fn test_hash64() {
        let input = "123
                     456
                     789";
        let mut map = CharMap::from_str_with_trim(input, '@');
        let original_hash = map.hash64();
        map.set_cell(0, 0, '.');
        assert_ne!(original_hash, map.hash64());
    }

    #[test]
    fn test_process() -> miette::Result<()> {
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
        assert_eq!("64", process(input)?);
        Ok(())
    }
}
