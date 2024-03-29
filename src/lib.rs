extern crate num_cpus;

use rayon::prelude::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

struct Point {
    x: i32,
    y: i32
}

struct Line {
    point1: Point,
    point2: Point,
}

type TPoint = Vec<i32>;
type TPoints = Vec<TPoint>;

fn get_distance_between_line_and_point(point: Point, line: Line) -> f32 {
    let x = point.x;
    let y = point.y;

    let x1 = line.point1.x;
    let y1 = line.point1.y;

    let x2 = line.point2.x;
    let y2 = line.point2.y;

    let double_area = ((y2 - y1) * x - (x2 - x1) * y + x2 * y1 - y2 * x1).abs() as f32;
    let line_segment_length = (((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f32).sqrt();

    if line_segment_length != 0.0 {
        return double_area / line_segment_length;
    }

    0.0
}

fn simplify_points(source_points: &TPoints, dest_points: &mut TPoints, tolerance: f32, begin: usize, end: usize) {
    if begin + 1 == end {
        return;
    }

    let mut max_index: usize = 0;
    let mut max_distance: f32 = -1.0;

    for i in begin + 1 .. end {
        let current_point = Point {
            x: source_points[i][0],
            y: source_points[i][1],
        };

        let line = Line {
            point1: Point {
                x: source_points[begin][0],
                y: source_points[begin][1],
            },
            point2: Point {
                x: source_points[end][0],
                y: source_points[end][1],
            },
        };

        let distance = get_distance_between_line_and_point(current_point, line);

        if distance > max_distance {
            max_index = i;
            max_distance = distance;
        }
    }

    if max_distance > tolerance {
        simplify_points(source_points, dest_points, tolerance, begin, max_index);

        dest_points.push(source_points[max_index].to_vec());

        simplify_points(source_points, dest_points, tolerance, max_index, end)
    }
}

#[pyfunction]
fn apply_to_points(points: TPoints, tolerance: f32) -> TPoints {
    let mut dest_points: TPoints = Vec::new();

    dest_points.push(points[0].to_vec());
    simplify_points(&points, &mut dest_points, tolerance, 0, (points.len() - 1).into());
    dest_points.push(points[points.len() - 1].to_vec());

    dest_points
}

#[pyfunction]
fn apply_to_lines(lines: Vec<TPoints>, tolerance: f32) -> Vec<TPoints> {
    lines.into_par_iter()
        .map(|i| apply_to_points(i, tolerance))
        .rev().collect()
}

#[pymodule]
fn douglas_peucker(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(apply_to_lines, m)?)?;
    m.add_function(wrap_pyfunction!(apply_to_points, m)?)?;

    Ok(())
}
