use crate::{
    custom_error::AocError,
    utils::{Direction, Point},
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut vertices = Vec::with_capacity(input.lines().count());

    let mut pos = Point::new(0, 0);
    let mut perimeter = 0;
    let mut directions = Vec::new();

    for line in input.lines() {
        let color = line.trim().split(' ').last().unwrap();
        let color = color.trim_start_matches('(').trim_end_matches(')');
        let (steps, dir) = parse_color(color);

        match dir {
            Direction::East => pos.x += steps,
            Direction::South => pos.y += steps,
            Direction::West => pos.x -= steps,
            Direction::North => pos.y -= steps,
        }

        perimeter += steps;
        vertices.push(pos);
        directions.push(dir);
    }

    let corner_count = directions.len() as i64;
    let area = polygon_area(&vertices);
    let corner_area = corner_area(&directions);
    let perimeter_without_corners = perimeter - corner_count;

    let result = area + (perimeter_without_corners / 2) + (corner_count - corner_area);
    Ok(result.to_string())
}

// For each corner, determines the area of the interior angle we have already accounted for.
// For an external corner, the area is 0.25.
// For an internal corner, the area is 0.75.
// Returns the sum of all corner areas.
fn corner_area(directions: &[Direction]) -> i64 {
    let mut sum = 0.0;

    for corner in 0..directions.len() {
        let prev = if corner == 0 {
            directions.len() - 1
        } else {
            corner - 1
        };
        let cur_dir = directions[corner];
        let prev_dir = directions[prev];

        let area = match (prev_dir, cur_dir) {
            (Direction::East, Direction::South) => 0.25, // external
            (Direction::East, Direction::North) => 0.75, // internal

            (Direction::West, Direction::South) => 0.75, // internal
            (Direction::West, Direction::North) => 0.25, // external

            (Direction::South, Direction::West) => 0.25, // external
            (Direction::South, Direction::East) => 0.75, // internal

            (Direction::North, Direction::East) => 0.25, // external
            (Direction::North, Direction::West) => 0.75, // internal

            _ => panic!("Unknown corner: {:?} {:?}", prev_dir, cur_dir),
        };

        sum += area;
    }

    return sum as i64;
}

fn polygon_area(vertices: &[Point<i64>]) -> i64 {
    let mut area = 0;

    for i in 0..vertices.len() {
        let (xi, yi) = (vertices[i].x, vertices[i].y);
        let (x_next, y_next) = if i == vertices.len() - 1 {
            (vertices[0].x, vertices[0].y)
        } else {
            (vertices[i + 1].x, vertices[i + 1].y)
        };

        area += (xi * y_next) - (yi * x_next);
    }

    (area / 2).abs()
}

fn parse_color(color: &str) -> (i64, Direction) {
    let distance_hex = color[1..6].to_string();
    let distance = i64::from_str_radix(&distance_hex, 16).unwrap();
    let dir_hex = color.chars().nth(6).unwrap();
    let dir = match dir_hex {
        '0' => Direction::East,
        '1' => Direction::South,
        '2' => Direction::West,
        '3' => Direction::North,
        _ => panic!("Unknown direction: {}", dir_hex),
    };
    (distance, dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corner_area() {
        let rectangle = vec![
            Direction::East,
            Direction::South,
            Direction::West,
            Direction::North,
        ];
        assert_eq!(1, corner_area(&rectangle));

        //  #######
        //  #.....#
        //  #.###.#
        //  #.#.#.#
        //  ###.###
        let pants = vec![
            Direction::East,
            Direction::South,
            Direction::West,
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::West,
            Direction::North,
        ];
        assert_eq!(3, corner_area(&pants));
    }

    #[test]
    fn test_polygon_area() {
        let points = vec![
            Point::new(0, 0),
            Point::new(2, 0),
            Point::new(2, 2),
            Point::new(0, 2),
        ];
        assert_eq!(4, polygon_area(&points));
    }

    #[test]
    fn test_parse_color() {
        assert_eq!((461937, Direction::East), parse_color("#70c710"));
        assert_eq!((56407, Direction::South), parse_color("#0dc571"));
        assert_eq!((577262, Direction::West), parse_color("#8ceee2"));
        assert_eq!((829975, Direction::North), parse_color("#caa173"));
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "R 6 (#70c710)
                     D 5 (#0dc571)
                     L 2 (#5713f0)
                     D 2 (#d2c081)
                     R 2 (#59c680)
                     D 2 (#411b91)
                     L 5 (#8ceee2)
                     U 2 (#caa173)
                     L 1 (#1b58a2)
                     U 2 (#caa171)
                     R 2 (#7807d2)
                     U 3 (#a77fa3)
                     L 2 (#015232)
                     U 2 (#7a21e3)";
        assert_eq!("952408144115", process(input)?);
        Ok(())
    }
}

// Submissions:
// 92291574648776 - too high
// 92291468914147 - correct
// 92291363177928 - too low
