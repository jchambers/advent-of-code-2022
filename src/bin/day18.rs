use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let droplet = LavaDroplet::from_str(fs::read_to_string(path)?.as_str())?;

        println!("Total surface area: {}", droplet.total_surface_area());
        println!("External surface area: {}", droplet.external_surface_area());

        Ok(())
    } else {
        Err("Usage: day18 INPUT_FILE_PATH".into())
    }
}

struct LavaDroplet {
    voxels: Vec<Voxel>,
    bounds: (usize, usize, usize),
}

impl FromStr for LavaDroplet {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let lava_voxels: Vec<(usize, usize, usize)> = string
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(
                |line| match line.split(',').collect::<Vec<&str>>().as_slice() {
                    [x, y, z] => Ok((x.parse()?, y.parse()?, z.parse()?)),
                    _ => Err("Could not parse line".into()),
                },
            )
            .collect::<Result<_, Box<dyn Error>>>()?;

        let bounds = lava_voxels
            .iter()
            .fold((0usize, 0usize, 0usize), |(b_x, b_y, b_z), (x, y, z)| {
                (b_x.max(*x + 1), b_y.max(*y + 1), b_z.max(*z + 1))
            });

        let mut droplet = LavaDroplet {
            voxels: vec![Voxel::ExternalAir; bounds.0 * bounds.1 * bounds.2],
            bounds,
        };

        lava_voxels.iter().for_each(|&(x, y, z)| {
            let index = droplet.index(x, y, z);
            droplet.voxels[index] = Voxel::Lava;
        });

        let mut unvisited_air_voxels = HashSet::new();

        for x in 0..droplet.bounds.0 {
            for y in 0..droplet.bounds.1 {
                for z in 0..droplet.bounds.2 {
                    if !matches!(
                        droplet.voxel(x as isize, y as isize, z as isize),
                        Voxel::Lava
                    ) {
                        unvisited_air_voxels.insert((x, y, z));
                    }
                }
            }
        }

        while !unvisited_air_voxels.is_empty() {
            let mut exploration_queue = vec![*unvisited_air_voxels.iter().next().unwrap()];
            let mut explored_group = vec![];
            let mut group_has_path_to_surface = false;

            while !exploration_queue.is_empty() {
                let (x, y, z) = exploration_queue.pop().unwrap();

                droplet
                    .neighbors(x, y, z)
                    .into_iter()
                    .filter(|(x, y, z)| {
                        *x >= 0
                            && *y >= 0
                            && *z >= 0
                            && (*x as usize) < droplet.bounds.0
                            && (*y as usize) < droplet.bounds.1
                            && (*z as usize) < droplet.bounds.2
                    })
                    .filter(|(x, y, z)| {
                        unvisited_air_voxels.contains(&(*x as usize, *y as usize, *z as usize))
                    })
                    .for_each(|(x, y, z)| {
                        exploration_queue.push((x as usize, y as usize, z as usize));

                        if x == 0
                            || y == 0
                            || z == 0
                            || (x as usize) == droplet.bounds.0
                            || (y as usize) == droplet.bounds.1
                            || (z as usize) == droplet.bounds.2
                        {
                            group_has_path_to_surface = true;
                        }
                    });

                unvisited_air_voxels.remove(&(x, y, z));
                explored_group.push((x, y, z));
            }

            explored_group.into_iter().for_each(|(x, y, z)| {
                let index = droplet.index(x, y, z);

                droplet.voxels[index] = if group_has_path_to_surface {
                    Voxel::ExternalAir
                } else {
                    Voxel::TrappedAir
                }
            });
        }

        Ok(droplet)
    }
}

impl LavaDroplet {
    fn index(&self, x: usize, y: usize, z: usize) -> usize {
        (self.bounds.0 * self.bounds.1 * z) + (self.bounds.0 * y) + x
    }

    fn voxel(&self, x: isize, y: isize, z: isize) -> Voxel {
        if x < 0
            || y < 0
            || z < 0
            || x as usize >= self.bounds.0
            || y as usize >= self.bounds.1
            || z as usize >= self.bounds.2
        {
            Voxel::ExternalAir
        } else {
            self.voxels[self.index(x as usize, y as usize, z as usize)]
        }
    }

    fn total_surface_area(&self) -> u32 {
        let mut surface_area = 0;

        for x in 0..self.bounds.0 {
            for y in 0..self.bounds.1 {
                for z in 0..self.bounds.2 {
                    if matches!(self.voxel(x as isize, y as isize, z as isize), Voxel::Lava) {
                        surface_area += self
                            .neighbors(x, y, z)
                            .iter()
                            .map(|&(neighbor_x, neighbor_y, neighbor_z)| {
                                self.voxel(neighbor_x, neighbor_y, neighbor_z)
                            })
                            .filter(|voxel| !matches!(voxel, Voxel::Lava))
                            .count() as u32;
                    }
                }
            }
        }

        surface_area
    }

    fn external_surface_area(&self) -> u32 {
        let mut surface_area = 0;

        for x in 0..self.bounds.0 {
            for y in 0..self.bounds.1 {
                for z in 0..self.bounds.2 {
                    if matches!(self.voxel(x as isize, y as isize, z as isize), Voxel::Lava) {
                        surface_area += self
                            .neighbors(x, y, z)
                            .iter()
                            .map(|&(neighbor_x, neighbor_y, neighbor_z)| {
                                self.voxel(neighbor_x, neighbor_y, neighbor_z)
                            })
                            .filter(|voxel| matches!(voxel, Voxel::ExternalAir))
                            .count() as u32;
                    }
                }
            }
        }

        surface_area
    }

    fn neighbors(&self, x: usize, y: usize, z: usize) -> [(isize, isize, isize); 6] {
        [
            (x as isize + 1, y as isize, z as isize),
            (x as isize - 1, y as isize, z as isize),
            (x as isize, y as isize + 1, z as isize),
            (x as isize, y as isize - 1, z as isize),
            (x as isize, y as isize, z as isize + 1),
            (x as isize, y as isize, z as isize - 1),
        ]
    }
}

#[derive(Copy, Clone)]
enum Voxel {
    ExternalAir,
    TrappedAir,
    Lava,
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_DROPLET: &str = indoc! {"
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
    "};

    #[test]
    fn test_total_surface_area() {
        {
            let droplet = LavaDroplet::from_str("1,1,1\n2,1,1").unwrap();
            assert_eq!(10, droplet.total_surface_area());
        }

        {
            let droplet = LavaDroplet::from_str(TEST_DROPLET).unwrap();
            assert_eq!(64, droplet.total_surface_area());
        }
    }

    #[test]
    fn test_external_surface_area() {
        {
            let droplet = LavaDroplet::from_str("1,1,1\n2,1,1").unwrap();
            assert_eq!(10, droplet.external_surface_area());
        }

        {
            let droplet = LavaDroplet::from_str(TEST_DROPLET).unwrap();
            assert_eq!(58, droplet.external_surface_area());
        }
    }
}
