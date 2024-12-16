use crate::Day;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter;
use std::rc::Rc;

pub struct Day12 {}

impl Day for Day12 {
    fn part1(&self, lines: &[&str]) -> i64 {
        let plants = parse(lines);
        let mut farm = Farm::new(plants.len());

        farm.create_regions(&plants);
        farm.calculate_area_perimeter();

        farm.regions
            .iter()
            .map(|region| {
                let r = region.borrow();
                r.area * r.perimeter
            })
            .sum::<u64>() as i64
    }

    fn part2(&self, lines: &[&str]) -> i64 {
        let plants = parse(lines);
        let mut farm = Farm::new(plants.len());

        farm.create_regions(&plants);
        farm.calculate_area_perimeter();
        farm.calculate_sides();

        for region in &farm.regions {
            println!("{} - {}", region.borrow().area, region.borrow().sides);
        }

        farm.regions
            .iter()
            .map(|region| {
                let r = region.borrow();
                r.area * r.sides
            })
            .sum::<u64>() as i64
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Pos { x, y }
    }
}

struct Region {
    id: u32,
    plant: char,
    area: u64,
    perimeter: u64,
    sides: u64,
}

impl PartialEq for Region {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Region {
    fn new(id: u32, plant: char) -> Self {
        Region {
            id,
            plant,
            area: 0,
            perimeter: 0,
            sides: 0,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum EdgeType {
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone)]
struct Edge {
    start: Pos,
    end: Pos,
    type_: EdgeType,
}

impl Edge {
    fn combine(&self, other: &Self) -> Option<Edge> {
        if self.type_ != other.type_ {
            return None;
        }

        if self.start == other.end {
            return Some(Edge {
                start: other.start,
                end: self.end,
                type_: self.type_,
            });
        }

        if self.end == other.start {
            return Some(Edge {
                start: self.start,
                end: other.end,
                type_: self.type_,
            });
        }

        None
    }
}

struct Farm {
    size: usize,
    regions: Vec<Rc<RefCell<Region>>>,
    gardens: HashMap<Pos, Rc<RefCell<Region>>>,
}

impl Farm {
    const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    fn new(size: usize) -> Farm {
        Farm {
            size,
            regions: Vec::new(),
            gardens: HashMap::new(),
        }
    }

    fn create_regions(&mut self, plants: &Vec<Vec<char>>) {
        let mut id = 0;

        for y in 0..self.size {
            for x in 0..self.size {
                if matches!(self.get_region(x, y), None) {
                    self.flood_fill_region(plants, id, x, y);
                    id += 1;
                }
            }
        }
    }

    fn flood_fill_region(&mut self, plants: &Vec<Vec<char>>, id: u32, x: usize, y: usize) {
        let plant = plants[y][x];
        let region = Rc::new(RefCell::new(Region::new(id, plant)));

        self.regions.push(region.clone());

        let mut pending = VecDeque::new();
        let mut explored = HashSet::new();

        pending.push_back(Pos::new(x, y));
        explored.insert(Pos::new(x, y));

        while pending.len() > 0 {
            let pos = pending.pop_front().unwrap();

            if plants[pos.y][pos.x] != plant {
                continue;
            }

            self.gardens.insert(pos, region.clone());

            for dir in Self::DIRECTIONS {
                if let Some(neighbour) = offset(pos.x, pos.y, dir, self.size) {
                    if !explored.contains(&neighbour) {
                        pending.push_back(neighbour);
                        explored.insert(neighbour);
                    }
                }
            }
        }
    }

    fn calculate_area_perimeter(&mut self) {
        for region in &self.regions {
            region.borrow_mut().area = self.region_gardens(region.clone()).count() as u64;
            region.borrow_mut().perimeter = self.region_edges(region.clone()).count() as u64;
        }
    }

    fn calculate_sides(&mut self) {
        for region in &self.regions {
            let mut edges = self.region_edges(region.clone()).collect::<Vec<Edge>>();
            let mut i = 0;

            while i < edges.len() {
                let mut j = i + 1;

                while j < edges.len() {
                    let edge = &edges[i];
                    let other = &edges[j];

                    if let Some(new) = edge.combine(&other) {
                        edges[i] = new;
                        edges.remove(j);
                    } else {
                        j += 1;
                    }
                }

                i += 1;
            }

            region.borrow_mut().sides = edges.len() as u64;
        }
    }

    fn region_edges(&self, region: Rc<RefCell<Region>>) -> impl Iterator<Item = Edge> + use<'_> {
        self.region_gardens(region.clone()).flat_map(move |pos| {
            let r = region.clone();

            iter::repeat_n(pos, 4)
                .zip(Self::DIRECTIONS)
                .filter(move |(pos, dir)| match offset(pos.x, pos.y, *dir, self.size) {
                    Some(neighbour_pos) => {
                        let mut ok = false;

                        if let Some(neighbour) = self.get_region(neighbour_pos.x, neighbour_pos.y) {
                            if neighbour.borrow().plant != r.borrow().plant {
                                ok = true;
                            }
                        }

                        ok
                    }
                    None => true,
                })
                .map(|(pos, dir)| Edge {
                    start: match dir {
                        (1, 0) => Pos::new(pos.x + 1, pos.y),
                        (-1, 0) => Pos::new(pos.x, pos.y),
                        (0, 1) => Pos::new(pos.x, pos.y + 1),
                        (0, -1) => Pos::new(pos.x, pos.y),
                        _ => panic!("Invalid direction"),
                    },
                    end: match dir {
                        (1, 0) => Pos::new(pos.x + 1, pos.y + 1),
                        (-1, 0) => Pos::new(pos.x, pos.y + 1),
                        (0, 1) => Pos::new(pos.x + 1, pos.y + 1),
                        (0, -1) => Pos::new(pos.x + 1, pos.y),
                        _ => panic!("Invalid direction"),
                    },
                    type_: match dir {
                        (1, 0) | (-1, 0) => EdgeType::Horizontal,
                        (0, 1) | (0, -1) => EdgeType::Vertical,
                        _ => panic!("Invalid direction"),
                    },
                })
        })
    }

    fn region_gardens(&self, region: Rc<RefCell<Region>>) -> impl Iterator<Item = &Pos> {
        self.gardens.iter().filter(move |(_, r)| **r == region).map(|(pos, _)| pos)
    }

    fn get_region(&self, x: usize, y: usize) -> Option<&Rc<RefCell<Region>>> {
        self.gardens.get(&Pos::new(x, y))
    }
}

fn offset(x: usize, y: usize, dir: (isize, isize), size: usize) -> Option<Pos> {
    let x = x as isize + dir.0;
    let y = y as isize + dir.1;

    if x >= 0 && x < size as isize && y >= 0 && y < size as isize {
        Some(Pos::new(x as usize, y as usize))
    } else {
        None
    }
}

fn parse(lines: &[&str]) -> Vec<Vec<char>> {
    lines.iter().map(|str| str.chars().collect()).collect()
}
