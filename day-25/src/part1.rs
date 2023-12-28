use coupe::Partition as _;
use sprs::CsMat;
use std::collections::HashMap;

use crate::custom_error::AocError;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut nodes: HashMap<String, usize> = HashMap::default();
    let mut node_id = 0;

    let mut adjacency = CsMat::empty(sprs::CSR, 0);

    for line in input.lines() {
        let mut parts = line.trim().split(':');
        let component = parts.next().unwrap().trim().to_string();
        let connected_to = parts.next().unwrap().trim().split(' ').collect::<Vec<_>>();

        for connection in connected_to {
            let connection = connection.trim().to_string();

            if !nodes.contains_key(&component) {
                nodes.insert(component.clone(), node_id);
                node_id += 1;
            }

            if !nodes.contains_key(&connection) {
                nodes.insert(connection.clone(), node_id);
                node_id += 1;
            }

            let source_id = nodes.get(&component).unwrap();
            let target_id = nodes.get(&connection).unwrap();

            // Add edge to adjacency matrix (undirected)
            adjacency.insert(*source_id, *target_id, 1);
            adjacency.insert(*target_id, *source_id, 1);
        }
    }

    let node_count = nodes.len();
    println!("Node count: {}", node_count);

    // Assign all nodes to one side
    let mut partition = vec![0; node_count];

    // Flip half of the nodes to the other side
    for i in 0..node_count / 2 {
        partition[i] = 1;
    }

    // All nodes have the same weight
    let weights = vec![1.; node_count];

    // Run the partitioning algorithm
    coupe::FiducciaMattheyses {
        max_imbalance: Some(0.25),
        max_bad_move_in_a_row: 10000,
        ..Default::default()
    }
    .partition(&mut partition, (adjacency.view(), &weights))
    .unwrap();

    let left_count = partition.iter().filter(|&&x| x == 0).count();
    let right_count = partition.iter().filter(|&&x| x == 1).count();
    println!("Left: {}, Right: {}", left_count, right_count);

    let result = left_count * right_count;
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        tracing_subscriber::fmt().init();

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
