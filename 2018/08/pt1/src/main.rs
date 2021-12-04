use std::fs::read_to_string;

struct Node<'a> {
    children: Vec<Node<'a>>,
    metadata: &'a [u32],
}

fn parse_node(data: &[u32]) -> (usize, Node) {
    if data.len() < 2 {
        panic!("Not enough numbers for header");
    }

    let child_count = data[0];
    let meta_count = data[1] as usize;

    let mut bytes_read = 2;
    let mut children = Vec::new();

    for _ in 0..child_count {
        let (read, node) = parse_node(&data[bytes_read..]);
        bytes_read += read;
        children.push(node);
    }

    let metadata = &data[bytes_read .. bytes_read + meta_count];
    bytes_read += meta_count;

    (bytes_read, Node {children, metadata})
}

fn sum_nodes(node: &Node) -> u32 {
    let c: u32 = node.children.iter()
        .map(sum_nodes)
        .sum();
    let m: u32 = node.metadata.iter()
        .sum();

    c + m
}

fn main() {
    let input = read_to_string("input").expect("could not load input file");

    let nums: Vec<u32> = input.trim().split_terminator(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    let (bytes_read, data) = parse_node(&nums);

    let sum = sum_nodes(&data);

    println!("{}", sum);
}
