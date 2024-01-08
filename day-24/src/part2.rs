use z3::{ast::Int, Config, Context, SatResult, Solver};

use crate::{custom_error::AocError, utils::Point3D};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut lines = Vec::new();

    for line in input.lines() {
        let mut parts = line.trim().split('@');
        let coords = parts.next().unwrap().trim();
        let speeds = parts.next().unwrap().trim();

        let p1 = Point3D::from_str(coords);
        let speed = Point3D::from_str(speeds);

        lines.push((p1, speed));
    }

    let mut smt32_lines: Vec<String> = vec![
        "(declare-const xr Int)".to_string(),
        "(declare-const yr Int)".to_string(),
        "(declare-const zr Int)".to_string(),
        "(declare-const vxr Int)".to_string(),
        "(declare-const vyr Int)".to_string(),
        "(declare-const vzr Int)".to_string(),
    ];

    for (h, vh) in &lines {
        smt32_lines.push(format!(
            r#"
                (assert (= (- (* (- {xh} xr) (- vyr {vyh})) (* (- vxr {vxh}) (- {yh} yr))) 0))
                (assert (= (- (* (- {yh} yr) (- vzr {vzh})) (* (- vyr {vyh}) (- {zh} zr))) 0))
            "#,
            xh = h.x,
            yh = h.y,
            zh = h.z,
            vxh = vh.x,
            vyh = vh.y,
            vzh = vh.z,
        ));
    }

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);
    let problem = smt32_lines.join("\n");
    solver.from_string(problem);

    assert_eq!(SatResult::Sat, solver.check());
    let model = solver.get_model().unwrap();

    let xr = model.eval(&Int::new_const(&ctx, "xr"), true).unwrap();
    let yr = model.eval(&Int::new_const(&ctx, "yr"), true).unwrap();
    let zr = model.eval(&Int::new_const(&ctx, "zr"), true).unwrap();

    let answer = xr.as_i64().unwrap() + yr.as_i64().unwrap() + zr.as_i64().unwrap();

    Ok(answer.to_string())
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
