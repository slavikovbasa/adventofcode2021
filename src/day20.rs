use std::{fmt::Display, str::FromStr};

#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/20/input";

struct Image {
    pixels: Vec<Vec<Pixel>>,
    outside: Pixel,
}

impl Image {
    fn height(&self) -> usize {
        self.pixels.len()
    }

    fn width(&self) -> usize {
        self.pixels.first().unwrap().len()
    }

    fn get(&self, (i, j): (i32, i32)) -> Pixel {
        if i >= 0 && j >= 0 && i < self.height() as i32 && j < self.width() as i32 {
            self.pixels[i as usize][j as usize]
        } else {
            self.outside
        }
    }

    fn count_lit(&self) -> usize {
        let mut count = 0;
        for row in self.pixels.iter() {
            for p in row {
                if let Pixel::Light = p {
                    count += 1;
                }
            }
        }
        count
    }

    fn enhanced(&self, (i, j): (i32, i32), algorithm: &Vec<Pixel>) -> Pixel {
        let ver = [i - 1, i, i + 1];
        let hor = [j - 1, j, j + 1];

        let mut kernel = ver
            .iter()
            .flat_map(|i| hor.iter().map(|j| self.get((*i, *j))));

        let mut num = kernel.next().unwrap().as_digit();
        kernel.for_each(|p| {
            num <<= 1;
            num |= p.as_digit();
        });

        algorithm[num]
    }

    fn enhance(&mut self, algorithm: &Vec<Pixel>) {
        let width = self.width();
        let height = self.height();
        let mut new_pixels = vec![vec![Pixel::Dark; width + 2]; height + 2];
        for i in 0..height + 2 {
            for j in 0..width + 2 {
                new_pixels[i][j] = self.enhanced((i as i32 - 1, j as i32 - 1), algorithm);
            }
        }
        self.outside = match self.outside {
            Pixel::Dark => algorithm[0b000000000],
            Pixel::Light => algorithm[0b111111111],
        };
        self.pixels = new_pixels;
    }
}

impl FromStr for Image {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pixels = s.lines().map(line_to_pixels).collect();

        Ok(Image {
            pixels,
            outside: Pixel::Dark,
        })
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        for row in self.pixels.iter() {
            for p in row {
                res.push_str(&format!("{}", p));
            }
            res.push('\n');
        }
        writeln!(f, "{}", res)
    }
}

#[derive(Clone, Copy)]
enum Pixel {
    Light,
    Dark,
}

impl Pixel {
    fn as_digit(&self) -> usize {
        match self {
            Pixel::Light => 1,
            Pixel::Dark => 0,
        }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Self::Light => '#',
            Self::Dark => '.',
        };
        write!(f, "{}", ch)
    }
}

fn line_to_pixels(line: &str) -> Vec<Pixel> {
    line.chars()
        .map(|c| match c {
            '#' => Pixel::Light,
            '.' => Pixel::Dark,
            _ => panic!("unexpected symbol {}", c),
        })
        .collect()
}

fn get_inputs(text: &str) -> (Vec<Pixel>, Image) {
    let mut parts = text.split("\n\n");
    let algorithm = parts.next().map(line_to_pixels).unwrap();

    let image = parts.next().unwrap().parse().unwrap();

    (algorithm, image)
}

#[allow(dead_code)]
pub fn solve1(text: &str) -> usize {
    let (algorithm, mut image) = get_inputs(text);

    image.enhance(&algorithm);
    image.enhance(&algorithm);

    image.count_lit()
}

#[allow(dead_code)]
pub fn solve2(text: &str) -> usize {
    let (algorithm, mut image) = get_inputs(text);

    for _ in 0..50 {
        image.enhance(&algorithm);
    }

    image.count_lit()
}
