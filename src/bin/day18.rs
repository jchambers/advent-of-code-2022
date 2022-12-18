use std::error::Error;
use std::fs;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let droplet = LavaDroplet::from_str(fs::read_to_string(path)?.as_str())?;

        println!("Droplet surface area: {}", droplet.surface_area());

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
            voxels: vec![Voxel::Air; bounds.0 * bounds.1 * bounds.2],
            bounds,
        };

        lava_voxels.iter().for_each(|&(x, y, z)| {
            let index = droplet.index(x, y, z);
            droplet.voxels[index] = Voxel::Lava;
        });

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
            Voxel::Air
        } else {
            self.voxels[self.index(x as usize, y as usize, z as usize)]
        }
    }

    fn surface_area(&self) -> u32 {
        let mut surface_area = 0;

        for x in 0..self.bounds.0 {
            for y in 0..self.bounds.1 {
                for z in 0..self.bounds.2 {
                    if matches!(self.voxel(x as isize, y as isize, z as isize), Voxel::Lava) {
                        surface_area += self.voxel_surface_area(x, y, z);
                    }
                }
            }
        }

        surface_area
    }

    fn voxel_surface_area(&self, x: usize, y: usize, z: usize) -> u32 {
        [
            (x as isize + 1, y as isize, z as isize),
            (x as isize - 1, y as isize, z as isize),
            (x as isize, y as isize + 1, z as isize),
            (x as isize, y as isize - 1, z as isize),
            (x as isize, y as isize, z as isize + 1),
            (x as isize, y as isize, z as isize - 1),
        ]
        .iter()
        .map(|&(neighbor_x, neighbor_y, neighbor_z)| self.voxel(neighbor_x, neighbor_y, neighbor_z))
        .filter(|voxel| matches!(voxel, Voxel::Air))
        .count() as u32
    }
}

#[derive(Copy, Clone)]
enum Voxel {
    Air,
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
    fn test_surface_area() {
        {
            let droplet = LavaDroplet::from_str("1,1,1\n2,1,1").unwrap();
            assert_eq!(10, droplet.surface_area());
        }

        {
            let droplet = LavaDroplet::from_str(TEST_DROPLET).unwrap();
            assert_eq!(64, droplet.surface_area());
        }
    }
}
