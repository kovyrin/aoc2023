use crate::custom_error::AocError;

fn aoc_hash(input: &str) -> u64 {
    let mut hash = 0;
    for c in input.chars() {
        hash = ((hash + c as u64) * 17) % 256;
    }
    return hash;
}

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: u64,
}

impl Lens {
    fn from_rule(rule: &str) -> (Lens, char) {
        let mut rule_len = rule.len();
        let mut focal_length = 0;
        if rule.chars().last().unwrap().is_numeric() {
            focal_length = rule[rule.len() - 1..].parse::<u64>().unwrap();
            rule_len = rule_len - 1;
        }

        let label = rule[..rule_len - 1].to_string();
        let operation = rule.chars().nth(rule_len - 1).unwrap();

        return (
            Lens {
                label,
                focal_length,
            },
            operation,
        );
    }

    fn box_number(&self) -> u64 {
        aoc_hash(&self.label)
    }
}

impl std::fmt::Debug for Lens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.label, self.focal_length)
    }
}

fn print_boxes(boxes: &Vec<Vec<Lens>>) {
    for i in 0..boxes.len() {
        if boxes[i].len() > 0 {
            println!("Box {}: {:?}", i, boxes[i]);
        }
    }
    println!();
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

    let rules = input.trim().split(",");
    for rule in rules {
        let (lens, operation) = Lens::from_rule(rule);
        println!("After {:?}", rule);
        handle_operation(&mut boxes, &lens, operation);
        print_boxes(&boxes);
    }

    let mut total_power = 0;
    for (box_number, bx) in boxes.iter().enumerate() {
        for (slot_number, lens) in bx.iter().enumerate() {
            let lens_power = (1 + box_number as u64) * (1 + slot_number as u64) * lens.focal_length;
            total_power = total_power + lens_power;
        }
    }
    return Ok(total_power.to_string());
}

fn handle_operation(boxes: &mut Vec<Vec<Lens>>, lens: &Lens, operation: char) {
    let box_number = lens.box_number();
    let current_box = boxes.get_mut(box_number as usize).unwrap();

    match operation {
        '-' => {
            let mut i = 0;
            while i < current_box.len() {
                if current_box[i].label == lens.label {
                    current_box.remove(i);
                } else {
                    i = i + 1;
                }
            }
        }
        '=' => {
            let mut replaced = false;
            for i in 0..current_box.len() {
                if current_box[i].label == lens.label {
                    current_box[i] = lens.clone();
                    replaced = true;
                    break;
                }
            }

            if !replaced {
                current_box.push(lens.clone());
            }
        }
        _ => {
            panic!("Unknown operation: {}", operation)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(52, aoc_hash("HASH"));
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("145", process(input)?);
        Ok(())
    }
}
