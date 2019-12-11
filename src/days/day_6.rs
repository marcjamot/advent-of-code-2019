use std::collections::HashMap;
use std::fs;

struct Node {
    name: String,
    children: HashMap<String, Node>,
}

pub fn run(input: &str) {
    let inputs = load_inputs(input);
    part_1(&inputs);
    part_2(&inputs);
}

fn load_inputs(file_name: &str) -> Node {
    let content = fs::read_to_string(file_name).expect("Could not read file");
    let lines = content.lines();

    let mut nodes: HashMap<&str, Node> = HashMap::new();
    let mut parents: HashMap<&str, &str> = HashMap::new();
    for line in lines {
        let node_names: Vec<&str> = line.split(")").collect();
        assert_eq!(node_names.len(), 2);
        let parent_node_name = node_names[0];
        let child_node_name = node_names[1];

        let child_node: Node;
        if nodes.contains_key(child_node_name) {
            child_node = nodes.remove(child_node_name).unwrap();
        } else {
            child_node = Node {
                name: String::from(child_node_name),
                children: HashMap::new(),
            };
        }

        let parent_node: &mut Node;
        if nodes.contains_key(parent_node_name) {
            parent_node = nodes.get_mut(parent_node_name).unwrap();
        } else if parents.contains_key(parent_node_name) {
            let mut paths = vec![parent_node_name];
            while !nodes.contains_key(paths.last().unwrap()) {
                paths.push(parents.get(paths.last().unwrap()).unwrap());
            }
            let mut root = nodes.get_mut(paths.pop().unwrap()).unwrap();
            paths.reverse();
            for path in paths {
                root = root.children.get_mut(path).unwrap();
            }
            parent_node = root;
        } else {
            let node = Node {
                name: String::from(parent_node_name),
                children: HashMap::new(),
            };
            nodes.insert(parent_node_name, node);
            parent_node = nodes.get_mut(parent_node_name).unwrap();
        }

        parent_node
            .children
            .insert(String::from(child_node_name), child_node);
        parents.insert(child_node_name, parent_node_name);
    }

    assert_eq!(nodes.len(), 1);
    return nodes.remove("COM").unwrap();
}

fn part_1(node: &Node) {
    let orbits = count_orbits(node, 0);
    println!("Part 1: {}", orbits);
}

fn count_orbits(node: &Node, depth: u32) -> u32 {
    let mut orbits = depth;
    for (_, child) in &node.children {
        orbits += count_orbits(child, depth + 1);
    }
    orbits
}

fn part_2(node: &Node) {
    println!("Part 2");
    let mut parent = node;

    let mut got_both = true;
    while got_both {
        got_both = false;
        for (_, child) in &parent.children {
            if find_child(child, "YOU", 0) != 0 && find_child(child, "SAN", 0) != 0 {
                parent = child;
                got_both = true;
                break;
            }
        }
    }

    println!("Common parent: {}", parent.name);
    let path_you = find_child(parent, "YOU", 0) - 1;
    let path_san = find_child(parent, "SAN", 0) - 1;
    println!(
        "Distance YOU = {}, distance SAN = {}, together: {}",
        path_you,
        path_san,
        path_you + path_san
    );
}

fn find_child(node: &Node, child_name: &str, depth: u32) -> u32 {
    if node.name == child_name {
        return depth;
    }

    for (_, child) in &node.children {
        let d = find_child(child, child_name, depth + 1);
        if d > 0 {
            return d;
        }
    }

    return 0;
}
