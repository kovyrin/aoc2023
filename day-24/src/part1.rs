use crate::{
    custom_error::AocError,
    utils::{Line, Point3D},
};

#[tracing::instrument]
pub fn process(input: &str, min: f64, max: f64) -> miette::Result<String, AocError> {
    let mut lines = Vec::new();

    for line in input.lines() {
        let mut parts = line.trim().split('@');
        let coords = parts.next().unwrap().trim();
        let speeds = parts.next().unwrap().trim();

        let p1 = Point3D::from_str(coords).xy();
        let speed = Point3D::from_str(speeds).xy();
        let p2 = p1 + speed;

        lines.push(Line::new(p1, p2));
    }

    let coord_range = min..=max;

    let mut count = 0;
    for l1 in 0..lines.len() - 1 {
        for l2 in l1 + 1..lines.len() {
            let line1 = &lines[l1];
            let line2 = &lines[l2];

            if let Some(intersect) = line1.intersects(line2) {
                if coord_range.contains(&intersect.x) && coord_range.contains(&intersect.y) {
                    let v1 = line1.vector();
                    let v2 = line1.p1.vector_to(&intersect);
                    let in_the_past = v1.x * v2.x + v1.y * v2.y <= 0.0;
                    if in_the_past {
                        println!(
                            "A={:?} intersects with B={:?} in the past for A",
                            line1, line2
                        );
                        continue;
                    }

                    let v1 = line2.vector();
                    let v2 = line2.p1.vector_to(&intersect);
                    let in_the_past = v1.x * v2.x + v1.y * v2.y <= 0.0;
                    if in_the_past {
                        println!(
                            "A={:?} intersects with B={:?} in the past for B",
                            line1, line2
                        );
                        continue;
                    }

                    println!("{:?} intersects with {:?} at {}", line1, line2, intersect);
                    count += 1;
                }
            }
        }
    }

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "19, 13, 30 @ -2,  1, -2
                     18, 19, 22 @ -1, -1, -2
                     20, 25, 34 @ -2, -2, -4
                     12, 31, 28 @ -1, -2, -1
                     20, 19, 15 @  1, -5, -3";
        assert_eq!("2", process(input, 7.0, 27.0)?);
        Ok(())
    }
}

// Submissions:
// 21562 - too low
// 29142 - correct
