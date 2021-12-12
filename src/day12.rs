use anyhow::{anyhow, Error, Result};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
enum NodeType {
    ONCE,
    TWICE,
    START,
    END,
    MANY,
}

impl NodeType {
    fn from_str(str: &str, twice: bool) -> Result<Self, Error> {
        if str.to_uppercase() == str {
            return Ok(NodeType::MANY);
        }
        if str == "start" {
            return Ok(NodeType::START);
        }
        if str == "end" {
            return Ok(NodeType::END);
        }
        if str.to_lowercase() == str {
            return Ok(if twice {
                NodeType::TWICE
            } else {
                NodeType::ONCE
            });
        }
        Err(anyhow!("Could not handle nodetype"))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node<'a> {
    id: &'a str,
    links: HashSet<&'a str>,
    node_type: NodeType,
}

impl<'a> Node<'a> {
    fn add_link(&mut self, node: &'a str) {
        self.links.insert(node);
    }
}

fn get_map<'a>(data: &'a str, twice: bool) -> Result<HashMap<&'a str, Node<'a>>> {
    let mut map: HashMap<&'a str, Node<'a>> = HashMap::new();
    let links = data
        .trim()
        .lines()
        .map(|line| {
            line.split_once("-")
                .map(Ok)
                .unwrap_or(Err(anyhow!("BAD FORMAT")))
        })
        .collect::<Result<Vec<(&str, &str)>>>()?;

    for pair in links.iter() {
        let (a, b): (&'a str, &'a str) = *pair;
        let node_a = map.entry(a).or_insert(Node {
            id: a,
            links: HashSet::new(),
            node_type: NodeType::from_str(a, twice)?,
        });
        node_a.add_link(b);
    }

    for pair in links.iter() {
        let (a, b): (&'a str, &'a str) = *pair;
        let node_b = map.entry(b).or_insert(Node {
            id: b,
            links: HashSet::new(),
            node_type: NodeType::from_str(b, twice)?,
        });
        node_b.add_link(a);
    }

    Ok(map)
}

fn walk<'a>(
    map: &HashMap<&str, Node<'a>>,
    mut done_visit: HashSet<&'a str>,
    mut done_visit_twice: HashSet<&'a str>,
    node: &Node<'a>,
) -> Result<usize> {
    if node.node_type == NodeType::END {
        return Ok(1);
    }

    match node.node_type {
        NodeType::MANY => (),
        NodeType::TWICE => {
            if done_visit.contains(node.id) && done_visit_twice.len() == 0 {
                done_visit_twice.insert(node.id);
            } else {
                done_visit.insert(node.id);
            }
        }
        _ => {
            done_visit.insert(node.id);
        }
    }

    let mut visits = 0;
    for n in node.links.iter() {
        let next_node = map
            .get(n)
            .map(Ok)
            .unwrap_or(Err(anyhow!("Did not find next node")))?;

        let skip = match next_node.node_type {
            NodeType::TWICE => {
                if done_visit_twice.len() == 0 {
                    false
                } else {
                    done_visit.contains(n)
                }
            }
            _ => done_visit.contains(n),
        };

        if skip {
            continue;
        } else {
            visits += walk(map, done_visit.clone(), done_visit_twice.clone(), next_node)?;
        }
    }

    Ok(visits)
}

fn part_one(data: &str) -> Result<usize> {
    let map = get_map(data, false)?;
    let start_node = map
        .get("start")
        .map(Ok)
        .unwrap_or(Err(anyhow!("Did not find next node")))?;
    walk(&map, HashSet::new(), HashSet::new(), &start_node)
}

fn part_two(data: &str) -> Result<usize> {
    let map = get_map(data, true)?;
    let start_node = map
        .get("start")
        .map(Ok)
        .unwrap_or(Err(anyhow!("Did not find next node")))?;
    walk(&map, HashSet::new(), HashSet::new(), &start_node)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let content = fs::read_to_string(path)?;

    Ok((part_one(&content)?, Some(part_two(&content)?)))
}

#[cfg(test)]
mod tests {
    use super::*;
    const SMALL_DATA: &str = "\n\
                              start-A\n\
                              start-b\n\
                              A-c\n\
                              A-b\n\
                              b-d\n\
                              A-end\n\
                              b-end\n\
                              ";

    #[test]
    fn test_walk() -> Result<()> {
        let map = get_map(SMALL_DATA, true)?;

        let start_node = map
            .get("start")
            .map(Ok)
            .unwrap_or(Err(anyhow!("Did not find next node")))?;

        let x = walk(&map, HashSet::new(), HashSet::new(), start_node)?;
        assert_eq!(x, 36);
        Ok(())
    }
}
