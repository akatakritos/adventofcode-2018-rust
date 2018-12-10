use std::fs;

pub fn metadata_checksum(license: &str) -> usize {
    let tree = get_tree_from_str(license);
    sum_metadata(&tree)
}

pub fn read_file(filename: &str) -> String {
    return fs::read_to_string(filename).unwrap();
}

fn get_tree_from_str(license: &str) -> LicenseNode {
    let license: Vec<usize> = license
        .split(' ')
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    create_tree(&license)
}

struct LicenseNode {
    metadata: Vec<usize>,
    children: Vec<Box<LicenseNode>>,
}

fn create_tree(license: &[usize]) -> LicenseNode {
    let mut index = 0;
    get_node(license, &mut index)
}

fn get_node(license: &[usize], index: &mut usize) -> LicenseNode {
    // println!("reading node starting at {}", index);

    let child_count = license[*index];
    *index += 1;

    let metadata_count = license[*index];
    *index += 1;

    // println!("reading {} children", child_count);
    let mut children = vec![];
    for _ in 0..child_count {
        children.push(Box::new(get_node(license, index)));
    }

    // println!("reading {} metadata fields", metadata_count);
    let mut metadata = vec![];
    for _ in 0..metadata_count {
        metadata.push(license[*index]);
        *index += 1;
    }

    // println!("metadata={:?} children count={}", metadata, children.len());

    LicenseNode {
        metadata,
        children,
    }
}

fn sum_metadata(root: &LicenseNode) -> usize {
    let mut sum = 0;

    for child in root.children.iter() {
        sum += sum_metadata(&child);
    }

    let sum_metadata: usize = root.metadata.iter().sum();

    sum + sum_metadata
}

pub fn calculate_node_value(license: &str) -> usize {
    let tree = get_tree_from_str(license);
    node_value(&tree)
}

fn node_value(node: &LicenseNode) -> usize {
    // if a node has no child nodes, its value is the sum of its metadata
    if node.children.len() == 0 {
        return node.metadata.iter().sum();
    }

    let mut value = 0;
    // if a child has children, metadata is indexes int the child list
    for index in node.metadata.iter() {
        // invalid indexes are skipped
        if *index == 0 || *index > node.children.len() {
            continue;
        }

        value += node_value(&node.children[index - 1]);
    }

    value
}

#[cfg(test)]
mod test {
    use spectral::prelude::*;
    use super::*;

    #[test]
    fn metadata_checksum_sample() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let checksum = metadata_checksum(input);
        assert_that!(checksum).is_equal_to(138);
    }

    #[test]
    fn metadata_checksum_input() {
        let input = read_file("inputs\\day08.txt");
        let checksum = metadata_checksum(&input);
        assert_that!(checksum).is_equal_to(45194);
    }

    #[test]
    fn node_value_sample() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let value = calculate_node_value(input);
        assert_that!(value).is_equal_to(66);
    }

    #[test]
    fn node_value_input() {
        let input = read_file("inputs\\day08.txt");
        let value = calculate_node_value(&input);
        assert_that!(value).is_equal_to(22989);
    }
}