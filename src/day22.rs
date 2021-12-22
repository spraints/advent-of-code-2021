use super::common::read_lines::read_lines;
use std::fmt::{Debug, Formatter};
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let mut world = new_world();
    for inst in read_lines(r).map(|line| parse_inst(&line)) {
        world.apply(inst);
    }

    let part1bounds = Cuboid {
        x: Extents { min: -50, max: 50 },
        y: Extents { min: -50, max: 50 },
        z: Extents { min: -50, max: 50 },
    };

    println!("part 1: {}", world.clamped(&part1bounds).count_on());
    println!("part 2: {}", world.count_on());
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Extents {
    min: isize,
    max: isize,
}

#[derive(Copy, Clone)]
struct Cuboid {
    x: Extents,
    y: Extents,
    z: Extents,
}

enum Instruction {
    On(Cuboid),
    Off(Cuboid),
}

fn parse_inst(line: &str) -> Instruction {
    let (c, p) = line.split_once(' ').unwrap();
    let cuboid = parse_cuboid(p);
    match c {
        "on" => Instruction::On(cuboid),
        "off" => Instruction::Off(cuboid),
        _ => panic!("could not parse: {}", line),
    }
}

// Input looks like "x=-31..18,y=-8..36,z=-8..37".
fn parse_cuboid(s: &str) -> Cuboid {
    let mut parts = s.split(',');
    let x = parse_extents(parts.next().unwrap());
    let y = parse_extents(parts.next().unwrap());
    let z = parse_extents(parts.next().unwrap());
    Cuboid { x, y, z }
}

// Input looks like "x=-31..18".
fn parse_extents(s: &str) -> Extents {
    let (_, range) = s.split_once('=').unwrap();
    let (min, max) = range.split_once("..").unwrap();
    Extents {
        min: min.parse().unwrap(),
        max: max.parse().unwrap(),
    }
}

fn new_world() -> World {
    World { on: vec![] }
}

struct World {
    on: Vec<Cuboid>,
}

impl World {
    fn count_on(&self) -> usize {
        self.on.iter().fold(0, |res, cuboid| res + cuboid.volume())
    }

    fn apply(&mut self, inst: Instruction) {
        match inst {
            Instruction::Off(off) => self.off(&off),
            Instruction::On(on) => {
                self.off(&on);
                self.on.push(on);
            }
        };
    }

    fn off(&mut self, off: &Cuboid) {
        self.on = self
            .on
            .iter()
            .flat_map(|c| {
                let res = c.delete(off);
                res
            })
            .collect();
    }

    fn clamped(&self, bounds: &Cuboid) -> World {
        World {
            on: self
                .on
                .iter()
                .filter_map(|cuboid| cuboid.intersect(bounds))
                .collect(),
        }
    }
}

impl Cuboid {
    fn volume(&self) -> usize {
        (self.x.max - self.x.min + 1) as usize
            * (self.y.max - self.y.min + 1) as usize
            * (self.z.max - self.z.min + 1) as usize
    }

    fn intersect(&self, bounds: &Cuboid) -> Option<Self> {
        let x = self.x.intersect(&bounds.x)?;
        let y = self.y.intersect(&bounds.y)?;
        let z = self.z.intersect(&bounds.z)?;
        Some(Self { x, y, z })
    }

    fn delete(&self, other: &Cuboid) -> Vec<Self> {
        // println!(
        //     "FROM {:?} (vol = {}) DELETE {:?}",
        //     self,
        //     self.volume(),
        //     other
        // );
        match self.intersect(other) {
            None => {
                // println!("-- no overlap!");
                vec![*self]
            }
            Some(other) => {
                // println!(
                //     "-- OVERLAP {:?} (vol = {}) (expected final volume = {})",
                //     other,
                //     other.volume(),
                //     self.volume() - other.volume()
                // );
                let mut res = vec![];
                let mut x = self.x;
                let mut y = self.y;
                let mut z = self.z;
                if other.x.min > x.min {
                    res.push(Cuboid {
                        x: Extents {
                            min: x.min,
                            max: other.x.min - 1,
                        },
                        y,
                        z,
                    });
                    x.min = other.x.min;
                }
                if other.y.min > y.min {
                    res.push(Cuboid {
                        x,
                        y: Extents {
                            min: y.min,
                            max: other.y.min - 1,
                        },
                        z,
                    });
                    y.min = other.y.min;
                }
                if other.z.min > z.min {
                    res.push(Cuboid {
                        x,
                        y,
                        z: Extents {
                            min: z.min,
                            max: other.z.min - 1,
                        },
                    });
                    z.min = other.z.min;
                }
                if other.z.max < z.max {
                    res.push(Cuboid {
                        x,
                        y,
                        z: Extents {
                            min: other.z.max + 1,
                            max: z.max,
                        },
                    });
                    z.max = other.z.max;
                }
                if other.y.max < y.max {
                    res.push(Cuboid {
                        x,
                        y: Extents {
                            min: other.y.max + 1,
                            max: y.max,
                        },
                        z,
                    });
                    y.max = other.y.max;
                }
                if other.x.max < x.max {
                    res.push(Cuboid {
                        x: Extents {
                            min: other.x.max + 1,
                            max: x.max,
                        },
                        y,
                        z,
                    });
                    //x.max = other.x.max;
                }

                // let mut new_vol = 0;
                // for c in res.iter() {
                //     println!(" ++ {:?}", c);
                //     new_vol += c.volume();
                // }
                // println!(" == new volume = {}", new_vol);

                res
            }
        }
    }
}

impl Debug for Cuboid {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Cuboid<[{},{}], [{},{}], [{},{}]>",
            self.x.min, self.x.max, self.y.min, self.y.max, self.z.min, self.z.max
        )
    }
}

impl Extents {
    fn intersect(&self, other: &Extents) -> Option<Self> {
        if self.min > other.max || other.min > self.max {
            None
        } else {
            let min = if self.min > other.min {
                self.min
            } else {
                other.min
            };
            let max = if self.max < other.max {
                self.max
            } else {
                other.max
            };
            Some(Self { min, max })
        }
    }
}

#[test]
fn test_extents_intersect() {
    let a = Extents { min: -10, max: 10 };
    let b = Extents { min: 3, max: 20 };
    let c = Extents { min: -20, max: -3 };
    let d = Extents { min: -5, max: 5 };

    assert_eq!(Extents { min: 3, max: 10 }, a.intersect(&b).unwrap());
    assert_eq!(Extents { min: -10, max: -3 }, a.intersect(&c).unwrap());
    assert_eq!(d, a.intersect(&d).unwrap());
    assert_eq!(d, d.intersect(&a).unwrap());

    assert_eq!(None, b.intersect(&c));
}
