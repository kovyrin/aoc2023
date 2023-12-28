use std::collections::{HashMap, HashSet, VecDeque};

use crate::custom_error::AocError;

type Graph = HashMap<String, HashMap<String, bool>>;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut links: Graph = HashMap::default();

    for line in input.lines() {
        let mut parts = line.trim().split(':');
        let component = parts.next().unwrap().trim().to_string();
        let connected_to = parts.next().unwrap().trim().split(' ').collect::<Vec<_>>();

        for connection in connected_to {
            let connection = connection.trim().to_string();

            links
                .entry(component.clone())
                .or_default()
                .insert(connection.clone(), true);

            links
                .entry(connection)
                .or_default()
                .insert(component.clone(), true);
        }
    }

    // Remove links (found through graphviz)
    // hrs - mnf
    // rkh - sph
    // kpc - nnl
    links.get_mut("hrs").unwrap().remove("mnf");
    links.get_mut("mnf").unwrap().remove("hrs");

    links.get_mut("rkh").unwrap().remove("sph");
    links.get_mut("sph").unwrap().remove("rkh");

    links.get_mut("kpc").unwrap().remove("nnl");
    links.get_mut("nnl").unwrap().remove("kpc");

    // Count nodes on each side of hrs-mnf link
    let left_count = count_nodes(&links, "hrs");
    let right_count = count_nodes(&links, "mnf");

    assert!(left_count > 0);
    assert!(right_count > 0);
    assert_eq!(left_count + right_count, links.len());

    println!("Left: {}", left_count);
    println!("Right: {}", right_count);

    let result = left_count * right_count;

    Ok(result.to_string())
}

fn count_nodes(links: &Graph, start: &str) -> usize {
    let mut visited: HashSet<String> = HashSet::default();
    let mut queue = VecDeque::default();
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        if visited.contains(node) {
            continue;
        }

        visited.insert(node.to_string());

        for neighbor in links.get(node).unwrap().keys() {
            queue.push_back(neighbor);
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "jqt: rhn xhk nvd
                     rsh: frs pzl lsr
                     xhk: hfx
                     cmg: qnr nvd lhk bvb
                     rhn: xhk bvb hfx
                     bvb: xhk hfx
                     pzl: lsr hfx nvd
                     qnr: nvd
                     ntq: jqt hfx bvb xhk
                     nvd: lhk
                     lsr: lhk
                     rzs: qnr cmg lsr rsh
                     frs: qnr lhk lsr";
        assert_eq!("54", process(input)?);
        Ok(())
    }
}

// Submissions:
// 614655 - correct
