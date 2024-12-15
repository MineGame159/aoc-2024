use crate::Day;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

pub struct Day10 {}

impl Day for Day10 {
    fn part1(&self, lines: &[&str]) -> i64 {
        let map = parse(lines);

        let mut sum = 0;

        for y in 0..lines.len() {
            for x in 0..lines[y].len() {
                if map[y][x] == 0 {
                    let graph = Graph::build(&map, x, y, |height| *height == 9, |from, to| (*from as i8) == (*to as i8) - 1);
                    sum += graph.ends.len() as i64;
                }
            }
        }

        sum
    }

    fn part2(&self, lines: &[&str]) -> i64 {
        let map = parse(lines);

        let mut sum = 0;

        for y in 0..lines.len() {
            for x in 0..lines[y].len() {
                if map[y][x] == 0 {
                    let graph = Graph::build(&map, x, y, |height| *height == 9, |from, to| (*from as i8) == (*to as i8) - 1);
                    sum += graph.get_path_count();
                }
            }
        }

        sum
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Node(u32);

impl Node {
    fn none() -> Self {
        Node(0)
    }

    fn valid(index: u32) -> Self {
        Node(index + 1)
    }

    fn is_valid(&self) -> bool {
        self.0 > 0
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const VALUES: [Self; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

    fn index(&self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }

    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

struct Graph {
    connections: Vec<[Node; 4]>,
    start: Node,
    ends: Vec<Node>,
}

impl Graph {
    fn build<T, FE, FT>(map: &Vec<Vec<T>>, x: usize, y: usize, is_end: FE, can_traverse: FT) -> Graph
    where
        T: Copy + Clone + Eq + Hash,
        FE: Fn(&T) -> bool,
        FT: Fn(&T, &T) -> bool,
    {
        let mut graph = Graph {
            connections: vec![],
            start: Node::none(),
            ends: vec![],
        };

        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        struct PosNode<'a, T> {
            pos: (usize, usize),
            node: Node,
            tile: &'a T,
        }

        let mut explored = HashMap::new();
        let mut ends = HashSet::new();
        let mut pending = VecDeque::new();

        graph.connections.push([Node::none(); 4]);
        graph.start = Node::valid(0);

        pending.push_back(PosNode {
            pos: (x, y),
            node: graph.start,
            tile: &map[y][x],
        });
        explored.insert((x, y), graph.start);

        while pending.len() > 0 {
            let pos_node = pending.pop_front().unwrap();

            if is_end(&pos_node.tile) {
                if !ends.contains(&pos_node.pos) {
                    ends.insert(pos_node.pos);
                    graph.ends.push(pos_node.node);
                }

                continue;
            }

            for dir in Direction::VALUES {
                let offset = dir.offset();

                let x = (pos_node.pos.0 as isize) + offset.0;
                let y = (pos_node.pos.1 as isize) + offset.1;

                if y >= 0 && y < map.len() as isize && x >= 0 && x < map[y as usize].len() as isize {
                    let x = x as usize;
                    let y = y as usize;

                    if can_traverse(&pos_node.tile, &map[y][x]) {
                        let neighbour = explored.get_mut(&(x, y));

                        if let Some(node) = neighbour {
                            let connections = &mut graph.connections[(node.0 as usize) - 1];
                            connections[dir.index()] = pos_node.node;
                        } else {
                            let mut connections = [Node::none(); 4];
                            connections[dir.index()] = pos_node.node;

                            graph.connections.push(connections);

                            let pos_node = PosNode {
                                pos: (x, y),
                                node: Node::valid((graph.connections.len() as u32) - 1),
                                tile: &map[y][x],
                            };

                            pending.push_back(pos_node);
                            explored.insert(pos_node.pos, pos_node.node);
                        }
                    }
                }
            }
        }

        graph
    }

    fn get_path_count(&self) -> i64 {
        let mut count = 0;

        struct Junction {
            connections: [Node; 4],
            next_i: Option<usize>,
        }

        impl Junction {
            fn next_node(&mut self) -> Option<Node> {
                let i = match self.next_i {
                    None => return None,
                    Some(i) => i,
                };

                self.next_i = match self.connections.iter().skip(i + 1).position(|conn| conn.is_valid()) {
                    Some(next_i) => Some(i + 1 + next_i),
                    None => None,
                };

                Some(self.connections[i])
            }
        }

        for end in &self.ends {
            let mut node = *end;
            let mut junctions = vec![];
            let mut backtracking = false;

            'outer: loop {
                // Backtrack

                while backtracking {
                    if junctions.len() == 0 {
                        break 'outer;
                    }

                    let junction: &mut Junction = junctions.last_mut().unwrap();

                    match junction.next_node() {
                        Some(n) => {
                            node = n;
                            backtracking = false;
                        }
                        None => {
                            junctions.pop().unwrap();
                        }
                    }
                }

                // Traverse

                if node == self.start {
                    count += 1;
                    backtracking = true;
                    continue;
                }

                let connections = self.get_connections(node);
                let count = connections.iter().filter(|conn| conn.is_valid()).count();

                if count == 0 {
                    backtracking = true;
                    continue;
                }

                if count == 1 {
                    node = *connections.iter().filter(|conn| conn.is_valid()).next().unwrap();
                    continue;
                }

                junctions.push(Junction {
                    connections,
                    next_i: connections.iter().position(|conn| conn.is_valid()),
                });

                node = junctions.last_mut().unwrap().next_node().unwrap();
            }
        }

        count
    }

    fn get_connections(&self, node: Node) -> [Node; 4] {
        self.connections[(node.0 as usize) - 1]
    }
}

fn parse(lines: &[&str]) -> Vec<Vec<u8>> {
    let mut map = Vec::with_capacity(lines.len());

    for y in 0..lines.len() {
        let mut row = Vec::with_capacity(lines[y].len());

        for x in 0..lines[y].len() {
            row.push(lines[y].chars().nth(x).unwrap().to_digit(10).unwrap() as u8);
        }

        map.push(row);
    }

    map
}
