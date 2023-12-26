use std::cmp::{max, min};

use crate::custom_error::AocError;

#[derive(Debug, Clone)]
struct World {
    bricks: Vec<Brick>,
}

impl World {
    // Let all the bricks settle down
    fn settle(&mut self) {
        // Sort the bricks by their z coordinate (lowest to highest)
        self.bricks.sort_by(|a, b| a.end.z.cmp(&b.end.z));

        // Iterate over the bricks and move them down until they can't move anymore
        let mut max_z_seen = self.bricks.first().unwrap().end.z;
        println!("Max z seen is {}", max_z_seen);

        for i in 0..self.bricks.len() {
            println!("Checking brick {}", i);

            // Move the brick down to at least the maximum z coordinate seen so far
            let move_by = self.bricks[i].end.z - max_z_seen - 1;
            if move_by > 0 {
                self.bricks[i].move_down(move_by)
            }

            // Keep moving the brick down while possible
            while self.can_move_down(&self.bricks[i]) {
                self.bricks[i].move_down(1);
            }

            println!(" - Brick {:?} has settled", self.bricks[i]);
            if self.bricks[i].start.z > max_z_seen {
                max_z_seen = self.bricks[i].start.z;
                println!(" - Max z seen is now {}", max_z_seen);
            }
        }

        // Sort the bricks by their z coordinate (lowest to highest)
        self.bricks.sort_by(|a, b| a.end.z.cmp(&b.end.z));
    }

    fn has_settled(&self) -> bool {
        for brick in &self.bricks {
            if self.can_move_down(brick) {
                println!("- Brick {:?} can move down", brick);
                return false;
            }
        }
        true
    }

    fn can_move_down(&self, brick: &Brick) -> bool {
        // Check if the brick is at the bottom of the world
        if brick.end.z == 1 {
            return false;
        }

        // Nearby bricks are those that have their z coordinate within 1 of the brick's z coordinates
        for other_brick in &self.bricks {
            if brick == other_brick {
                continue;
            }

            if other_brick.end.z > brick.start.z {
                continue;
            }

            if other_brick.start.z < brick.end.z - 1 {
                continue;
            }

            for p1 in brick.points() {
                for p2 in other_brick.points() {
                    if p1.on_top_of(&p2) {
                        return false;
                    }
                }
            }
        }

        true
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn from_str(s: &str) -> Self {
        let mut parts = s.split(',');
        let x = parts.next().unwrap().parse::<i32>().unwrap();
        let y = parts.next().unwrap().parse::<i32>().unwrap();
        let z = parts.next().unwrap().parse::<i32>().unwrap();
        Self::new(x, y, z)
    }

    fn on_top_of(&self, p2: &Point3D) -> bool {
        self.x == p2.x && self.y == p2.y && self.z == p2.z + 1
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Brick {
    name: char,
    start: Point3D,
    end: Point3D,
}

impl Brick {
    fn new(name: char, start: Point3D, end: Point3D) -> Self {
        // Make sure the points are ordered by their z coordinate (start should be higher)
        let (start, end) = if start.z > end.z {
            (start, end)
        } else {
            (end, start)
        };
        Self { name, start, end }
    }

    fn move_down(&mut self, move_by: i32) {
        if move_by == 0 {
            return;
        }

        println!(" - Moving brick {:?} down by {}", self, move_by);
        self.start.z -= move_by;
        self.end.z -= move_by;
    }

    // Breaks up the brick into a list of discrete 1x1x1 points
    fn points(&self) -> Vec<Point3D> {
        let mut points = Vec::new();

        // Iterate over the x, y, and z coordinates of the brick
        // Since x and y coordinates can be in arbitrary order, we need to account for that
        let start_x = min(self.start.x, self.end.x);
        let end_x = max(self.start.x, self.end.x);

        let start_y = min(self.start.y, self.end.y);
        let end_y = max(self.start.y, self.end.y);

        // Z coordinates are always ordered from start to end
        for z in self.end.z..=self.start.z {
            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    points.push(Point3D::new(x, y, z));
                }
            }
        }
        points
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut world = World { bricks: Vec::new() };

    let mut name = 'A';
    for line in input.lines() {
        let mut parts = line.trim().split('~');
        let left_point = Point3D::from_str(parts.next().unwrap());
        let right_point = Point3D::from_str(parts.next().unwrap());
        world.bricks.push(Brick::new(name, left_point, right_point));
        name = (name as u8 + 1) as char;
    }

    // Let all the bricks settle down
    world.settle();
    println!("Initial world has settled");

    // Make sure settling is done
    if !world.has_settled() {
        panic!("Bricks are still moving");
    }

    // Count all bricks that, if removed, would not cause any other brick to fall
    let mut count = 0;
    for brick in &world.bricks {
        println!("Checking brick {:?}", brick);
        let mut world_without_brick = world.clone();
        world_without_brick.bricks.retain(|b| b != brick);
        if world_without_brick.has_settled() {
            count += 1;
        }
    }

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_on_top_of() {
        let p1 = Point3D::new(0, 0, 1);
        let p2 = Point3D::new(0, 0, 0);
        assert!(p1.on_top_of(&p2));

        let p1 = Point3D::new(0, 1, 0);
        let p2 = Point3D::new(0, 0, 0);
        assert!(!p1.on_top_of(&p2));
    }

    #[test]
    fn test_brick_points() {
        // A line like 2,2,2~2,2,2 means that both ends of the brick are at the same coordinate - in other words, that the brick is a single cube.
        let brick = Brick::new('A', Point3D::new(0, 0, 0), Point3D::new(0, 0, 0));
        assert_eq!(vec![Point3D::new(0, 0, 0)], brick.points());

        // Lines like 0,0,10~1,0,10 or 0,0,10~0,1,10 both represent bricks that are two cubes in volume, both oriented horizontally.
        // The first brick extends in the x direction, while the second brick extends in the y direction.
        let brick1 = Brick::new('A', Point3D::new(0, 0, 10), Point3D::new(1, 0, 10));
        let brick2 = Brick::new('A', Point3D::new(0, 0, 10), Point3D::new(0, 1, 10));
        assert_eq!(2, brick1.points().len());
        assert_eq!(2, brick2.points().len());

        // A line like 0,0,1~0,0,10 represents a ten-cube brick which is oriented vertically. One end of the brick is the cube located at 0,0,1,
        // while the other end of the brick is located directly above it at 0,0,10.
        let brick = Brick::new('A', Point3D::new(0, 0, 1), Point3D::new(0, 0, 10));
        assert_eq!(10, brick.points().len());
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1,0,1~1,2,1
                     0,0,2~2,0,2
                     0,2,3~2,2,3
                     0,0,4~0,2,4
                     2,0,5~2,2,5
                     0,1,6~2,1,6
                     1,1,8~1,1,9";
        assert_eq!("5", process(input)?);
        Ok(())
    }
}

// Submissions:
// 393 - correct
