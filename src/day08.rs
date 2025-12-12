use std::{cmp::Ordering, collections::BinaryHeap, time::Instant};

use bitvec::order::Lsb0;
use bitvec::vec::BitVec;
use petgraph::algo::kosaraju_scc;
use petgraph::{Graph, graph::NodeIndex};

struct Position {
    x: u32,
    y: u32,
    z: u32,
}

impl Position {
    pub fn dist(&self, other: &Self) -> u64 {
        let diff_x = self.x as i64 - other.x as i64;
        let diff_y = self.y as i64 - other.y as i64;
        let diff_z = self.z as i64 - other.z as i64;
        (diff_x.pow(2) + diff_y.pow(2) + diff_z.pow(2)) as u64
    }
}

impl TryFrom<&str> for Position {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut nums_iter = value.split(",").map(str::parse).map(Result::ok);
        let Some(x) = nums_iter.next().flatten() else {
            return Err(());
        };
        let Some(y) = nums_iter.next().flatten() else {
            return Err(());
        };
        let Some(z) = nums_iter.next().flatten() else {
            return Err(());
        };
        Ok(Position { x, y, z })
    }
}

fn parse_input(input_data: String) -> Option<Vec<Position>> {
    input_data
        .lines()
        .map(Position::try_from)
        .map(Result::ok)
        .collect()
}

struct DistElem<const REV_ORD: bool> {
    distance: u64,
    a: usize,
    b: usize,
}

impl<const REV_ORD: bool> PartialEq for DistElem<REV_ORD> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<const REV_ORD: bool> Eq for DistElem<REV_ORD> {}

impl<const REV_ORD: bool> PartialOrd for DistElem<REV_ORD> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if REV_ORD {
            // we want to have the order in the heap by minimal distance
            self.distance
                .partial_cmp(&other.distance)
                .map(Ordering::reverse)
        } else {
            self.distance.partial_cmp(&other.distance)
        }
    }
}

impl<const REV_ORD: bool> Ord for DistElem<REV_ORD> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn solve_task1(boxes: &Vec<Position>, num_pairs: usize) -> usize {
    let mut dist_heap = (0..boxes.len())
        .flat_map(|i| {
            let a = &boxes[i];
            (i + 1..boxes.len()).map(move |j| {
                let b = &boxes[j];
                DistElem::<true> {
                    distance: a.dist(b),
                    a: i,
                    b: j,
                }
            })
        })
        .collect::<BinaryHeap<_>>();

    let mut graph = Graph::new_undirected();

    for idx in 0..boxes.len() {
        let node_idx = graph.add_node(());
        assert_eq!(idx, node_idx.index());
    }

    for _ in 0..num_pairs {
        let Some(min_edge) = dist_heap.pop() else {
            break;
        };

        let a_node = NodeIndex::new(min_edge.a);
        let b_node = NodeIndex::new(min_edge.b);

        graph.add_edge(a_node, b_node, ());
    }

    let mut conn_comp = kosaraju_scc(&graph);
    conn_comp.sort_unstable_by_key(|cc| -(cc.len() as i64));

    conn_comp.iter().take(3).map(Vec::len).product()
}

fn solve_task1_smart(boxes: &Vec<Position>, num_pairs: usize) -> usize {
    let mut ccs = (0..boxes.len())
        .map(|i| {
            let mut bitvec = BitVec::<usize, Lsb0>::with_capacity(boxes.len());
            if i > 0 {
                bitvec.extend_from_bitslice(&BitVec::<usize, Lsb0>::repeat(false, i));
            }
            bitvec.push(true);
            if i + 1 != boxes.len() {
                bitvec.extend_from_bitslice(&BitVec::<usize, Lsb0>::repeat(
                    false,
                    boxes.len() - i - 1,
                ));
            }
            bitvec
        })
        .collect::<Vec<_>>();

    let mut dist_heap = (0..boxes.len())
        .flat_map(|i| {
            let a = &boxes[i];
            (i + 1..boxes.len()).map(move |j| {
                let b = &boxes[j];
                DistElem::<true> {
                    distance: a.dist(b),
                    a: i,
                    b: j,
                }
            })
        })
        .collect::<BinaryHeap<_>>();

    let mut edge_count = 0;
    while edge_count < num_pairs
        && let Some(edge) = dist_heap.pop()
    {
        // this is basically UnionFind, but with stripped out connections, because we are not interested in those
        // only if the nodes are connected and not how
        let find_cc_idx = |bit_idx: usize| {
            for (cc_idx, cc) in ccs.iter().enumerate() {
                if *cc.get(bit_idx).unwrap() {
                    return cc_idx;
                }
            }
            unreachable!();
        };

        let idx_a = find_cc_idx(edge.a);
        let idx_b = find_cc_idx(edge.b);
        if idx_a != idx_b {
            let min_idx = idx_a.min(idx_b);
            let max_idx = idx_a.max(idx_b);
            let max_cc = ccs.remove(max_idx);
            ccs[min_idx] |= max_cc;
        }
        edge_count += 1;
    }

    let mut max1 = 0;
    let mut max2 = 0;
    let mut max3 = 0;

    for cc in &ccs {
        let l = cc.count_ones();
        if l > max1 {
            max3 = max2;
            max2 = max1;
            max1 = l;
        } else if l > max2 {
            max3 = max2;
            max2 = l;
        } else if l > max3 {
            max3 = l;
        }
    }

    max1 * max2 * max3
}

fn solve_task2(boxes: &Vec<Position>) -> u32 {
    let mut dist_heap = (0..boxes.len())
        .flat_map(|i| {
            let a = &boxes[i];
            (i + 1..boxes.len()).map(move |j| {
                let b = &boxes[j];
                DistElem::<true> {
                    distance: a.dist(b),
                    a: i,
                    b: j,
                }
            })
        })
        .collect::<BinaryHeap<_>>();

    let mut graph = Graph::new_undirected();

    for idx in 0..boxes.len() {
        let node_idx = graph.add_node(());
        assert_eq!(idx, node_idx.index());
    }

    assert!(kosaraju_scc(&graph).len() > 2);
    while let Some(min_edge) = dist_heap.pop() {
        let a_node = NodeIndex::new(min_edge.a);
        let b_node = NodeIndex::new(min_edge.b);

        graph.add_edge(a_node, b_node, ());
        if kosaraju_scc(&graph).len() == 1 {
            return boxes[min_edge.a].x * boxes[min_edge.b].x;
        }
    }

    unreachable!()
}

fn solve_task2_smart(boxes: &Vec<Position>) -> u32 {
    let mut ccs = (0..boxes.len())
        .map(|i| {
            let mut bitvec = BitVec::<usize, Lsb0>::with_capacity(boxes.len());
            if i > 0 {
                bitvec.extend_from_bitslice(&BitVec::<usize, Lsb0>::repeat(false, i));
            }
            bitvec.push(true);
            if i + 1 != boxes.len() {
                bitvec.extend_from_bitslice(&BitVec::<usize, Lsb0>::repeat(
                    false,
                    boxes.len() - i - 1,
                ));
            }
            bitvec
        })
        .collect::<Vec<_>>();

    let mut pairs = (0..boxes.len())
        .flat_map(|i| {
            let a = &boxes[i];
            (i + 1..boxes.len()).map(move |j| {
                let b = &boxes[j];
                DistElem::<false> {
                    distance: a.dist(b),
                    a: i,
                    b: j,
                }
            })
        })
        .collect::<Vec<_>>();
    pairs.sort_unstable();

    for edge in &pairs {
        // this is basically UnionFind, but with stripped out connections, because we are not interested in those
        // only if the nodes are connected and not how
        let find_cc_idx = |bit_idx: usize| {
            for (cc_idx, cc) in ccs.iter().enumerate() {
                if *cc.get(bit_idx).unwrap() {
                    return cc_idx;
                }
            }
            unreachable!();
        };

        let idx_a = find_cc_idx(edge.a);
        let idx_b = find_cc_idx(edge.b);
        if idx_a != idx_b {
            let min_idx = idx_a.min(idx_b);
            let max_idx = idx_a.max(idx_b);
            let max_cc = ccs.remove(max_idx);
            ccs[min_idx] |= max_cc;
        }

        if ccs.len() == 1 {
            return boxes[edge.a].x * boxes[edge.b].x;
        }
    }

    unreachable!()
}

pub fn solve_day08(input_data: String) {
    let input = match parse_input(input_data) {
        Some(input) => input,
        None => {
            eprintln!("Could not parse input!");
            return;
        }
    };

    let start_1 = Instant::now();
    let sol_task1 = solve_task1(&input, 1000);
    let end_1 = Instant::now();
    println!(
        "Task 1:\t\t{sol_task1}\t\tTook: {}µs",
        (end_1 - start_1).as_micros()
    );

    let start_1_smart = Instant::now();
    let sol_task1_smart = solve_task1_smart(&input, 1000);
    let end_1_smart = Instant::now();
    println!(
        "Smart 1:\t{sol_task1_smart}\t\tTook: {}µs",
        (end_1_smart - start_1_smart).as_micros()
    );

    let start_2 = Instant::now();
    let sol_task2 = solve_task2(&input);
    let end_2 = Instant::now();
    println!(
        "Task 2:\t\t{sol_task2}\tTook: {}µs",
        (end_2 - start_2).as_micros()
    );

    let start_2_smart = Instant::now();
    let sol_task2_smart = solve_task2_smart(&input);
    let end_2_smart = Instant::now();
    println!(
        "Smart 2:\t{sol_task2_smart}\tTook: {}µs",
        (end_2_smart - start_2_smart).as_micros()
    );
}
