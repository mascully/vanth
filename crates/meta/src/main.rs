//!

use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn add_axis_1(&self, amount: f32) -> Self {
        let angle = 4.0 / 6.0 * PI;
        (self.x + amount * angle.cos(), self.y + amount * angle.sin()).into()
    }

    fn add_axis_2(&self, amount: f32) -> Self {
        let angle = 2.0 / 6.0 * PI;
        (self.x + amount * angle.cos(), self.y + amount * angle.sin()).into()
    }

    fn to_string(&self) -> String {
        format!("{},{}", self.x, self.y)
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<f32> for Point {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Div<f32> for Point {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::Neg for Point {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Point { x: -self.x, y: -self.y }
    }
}

impl From<(f32, f32)> for Point {
    fn from((x, y): (f32, f32)) -> Self {
        Point { x, y }
    }
}

impl From<Point> for (f32, f32) {
    fn from(pt: Point) -> Self {
        (pt.x, pt.y)
    }
}

fn axonometric(x: f32, y: f32) -> Point {
    let dx = (2.0 / 6.0 * PI).cos();
    let dy = (2.0 / 6.0 * PI).sin();
    (50.0 + x * dx + y * dy, 50.0 + x * -dy + y * dx).into()
}

fn main() {
    let axis_1_dx = (2.0 / 6.0 * PI).cos();
    let axis_1_dy = (2.0 / 6.0 * PI).sin();
    let axis_2_dx = (4.0 / 6.0 * PI).cos();
    let axis_2_dy = (4.0 / 6.0 * PI).sin();

    // let mut point: Point = (20.0, 20.0).into();
    // let mut poly_1 = vec![point];
    // for &diff in &[
    //     (axis_1_dx * 10.0, axis_1_dy * 10.0),
    //     (10.0, 0.0),
    //     (axis_1_dx * -10.0, axis_1_dy * -10.0),
    // ] {
    //     point += <_ as Into<Point>>::into(diff);
    //     poly_1.push(point);
    // }

    // let mut point: Point = (20.0 + axis_1_dx * 60.0, 20.0).into();
    // let mut poly_2 = vec![point];
    // for &diff in &[
    //     (axis_2_dx * 10.0, axis_2_dy * 10.0),
    //     (10.0, 0.0),
    //     (axis_2_dx * -10.0, axis_2_dy * -10.0),
    // ] {
    //     point += <_ as Into<Point>>::into(diff);
    //     poly_2.push(point);
    // }

    // let mut point: Point = (23.0 + axis_1_dx * 3.0, 20.0 + axis_1_dy * 3.0).into();
    // let mut poly_3 = vec![point];
    // for &diff in &[
    //     (axis_1_dx * 30.0, axis_1_dy * 30.0),
    //     (axis_2_dx * -30.0, axis_2_dy * -30.0),
    //     (-10.0, 0.0),
    //     (axis_2_dx * 10.0, axis_2_dy * 10.0),
    //     (axis_1_dx * -10.0, axis_1_dy * -10.0),
    // ] {
    //     point += <_ as Into<Point>>::into(diff);
    //     poly_3.push(point);
    // }

    // let mut poly_4 = vec![
    //     (20.0 + axis_1_dx * 25.0, 20.0 + axis_1_dy * 25.0).into(),
    //     (50.0 - axis_2_dx * 25.0, 20.0 + axis_1_dy * 25.0).into(),
    //     (20.0 + axis_1_dx * 25.0, 25.0 + axis_1_dy * 25.0).into(),
    // ];
    
    let poly_1 = vec![
        axonometric(10.0, 10.0),
        axonometric(10.0, 20.0),
        axonometric(20.0, 20.0),
    ];
    
    let poly_2 = vec![];
    
    let poly_3 = vec![];
    
    let poly_4 = vec![];

    let mut svg = String::new();

    svg.push_str(
        r##"<?xml version="1.0" encoding="UTF-8"?>
        <svg xmlns="http://www.w3.org/2000/svg" width="500" height="500" viewBox="0 0 128 128">
        <rect width="128" height="128" fill="#ffffff"/>
        "##,
    );

    let blue = "#0b5cff";
    let black = "#000000";

    for (color, points) in &[(blue, poly_1), (blue, poly_2), (blue, poly_4), (black, poly_3)] {
        svg.push_str(&format!(r##"<g fill="{}">"##, color));
        svg.push_str(r##"<polygon points=""##);
        svg.push_str(&points.iter().map(Point::to_string).collect::<Vec<_>>().join(" "));
        svg.push_str(r##""/>"##);
        svg.push_str(r##"</g>"##);
    }

    svg.push_str(r##"</svg>"##);

    println!("{}", svg);
}
