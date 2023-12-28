use crate::{
    custom_error::AocError,
    utils::{Line, Point3D},
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
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

    // TODO: Solve maybe?

    Ok("".to_string())
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
        assert_eq!("47", process(input)?);
        Ok(())
    }
}
