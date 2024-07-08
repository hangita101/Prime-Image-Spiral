use std::collections::linked_list;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct Point {
    x: f64,
    y: f64,
}
struct Polar {
    r: f64,
    theta: f64,
}

impl Polar {
    fn to_point(&self) -> Point {
        Point {
            x: self.r * self.theta.sin(),
            y: self.r * self.theta.cos(),
        }
    }
}

fn read_file1(path: String, primes: &mut Vec<f64>) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let num: f64 = line?.trim().parse().unwrap();

        primes.push(num);
    }
    Ok(())
}

fn list_of_polar_from_float(nums: Vec<f64>) -> Vec<Polar> {
    let mut polar = Vec::new();
    for i in nums {
        polar.push(Polar { r: i, theta: i });
    }

    polar
}

fn list_of_cartesian_from_polar(pointsP: Vec<Polar>) -> Vec<Point> {
    let mut cartesian = Vec::new();
    for i in pointsP {
        cartesian.push(i.to_point());
    }
    cartesian
}

fn main() {
    let mut v_prime = Vec::new();
    read_file1("Final.txt".to_string(), &mut v_prime);

    let polar = list_of_polar_from_float(v_prime);
    let points = list_of_cartesian_from_polar(polar);

    for i in points {
        println!("{},{}", i.x, i.y);
    }
}
