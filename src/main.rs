use plotters::prelude::*;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct Point {
    x: f32,
    y: f32,
}
struct Polar {
    r: f32,
    theta: f32,
}

impl Polar {
    fn to_point(&self) -> Point {
        Point {
            x: self.r * self.theta.sin(),
            y: self.r * self.theta.cos(),
        }
    }
}

fn read_file1(path: String, primes: &mut Vec<f32>) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let num: f32 = line?.trim().parse().unwrap();

        primes.push(num);
    }
    Ok(())
}

fn list_of_polar_from_float(nums: Vec<f32>) -> Vec<Polar> {
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
fn list_of_tuple_from_cartesian(cartesian: Vec<Point>) -> Vec<(f32, f32)> {
    let mut lst = Vec::new();
    for i in cartesian {
        lst.push((i.x, i.y));
    }
    lst
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Destination File name
    let target_file_path = &args[1];
    let prime_file = &args[2];

    let mut v_prime = Vec::new();
    read_file1(prime_file.to_string(), &mut v_prime);

    let polar = list_of_polar_from_float(v_prime);
    let points = list_of_cartesian_from_polar(polar);

    let root = BitMapBackend::new(target_file_path, (10000, 10000)).into_drawing_area();
    root.fill(&BLACK);
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("Spiral Prime?", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(-100000f32..100000f32, -100000f32..100000f32)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    // And we can draw something in the drawing area
    // chart.draw_series(LineSeries::new(
    //     vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)],
    //     &RED,
    // ))?;
    // Similarly, we can draw point series

    let lst = list_of_tuple_from_cartesian(points);

    chart.draw_series(PointSeries::of_element(lst, 3, &GREEN, &|c, s, st| {
        EmptyElement::at(c) + Circle::new((0, 0), s, st.filled())
    }))?;

    root.present()?;
    Ok(())
}
