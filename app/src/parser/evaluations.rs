use crate::parser::symbolic::parse_complex_calculation;
use num_complex::Complex;
use std::collections::HashMap;

pub fn evaulate_points_on_contour(
    algorithm: Vec<String>,
    x_interval: (i32, i32),
    x_precision: i32,
    contour: Vec<String>,
) -> Vec<(Complex<f32>, i32, i32)> {
    let mut points = vec![];

    // Loop x interval
    for i in x_interval.0 * x_precision..x_interval.1 * x_precision + 1 {
        let x = i as f32 / x_precision as f32;
        // Create new contour with x change
        let mut contour_algorithm = vec![];
        contour.iter().for_each(|v| {
            if v == "x" {
                contour_algorithm.push(x.to_string());
            } else {
                contour_algorithm.push(v.clone());
            }
        });
        let y = parse_complex_calculation(contour_algorithm);

        if y.is_none() {
            break;
        }

        let y = y.unwrap().re;

        // format complex number in reverse polish notation
        let z = Complex::new(x, y);
        let z = format!("{}+i{}", z.re, z.im);
        let mut z: Vec<String> = z.split("+").map(|s| s.to_string()).collect();
        z.push("+".to_string());

        // Create new algorithm with z change
        let mut algorithm_z_change = vec![];
        algorithm.iter().for_each(|v| {
            if v == "z" {
                algorithm_z_change.append(&mut z.clone());
            } else {
                algorithm_z_change.push(v.clone());
            }
        });

        // Calculate new algorithm
        let z = parse_complex_calculation(algorithm_z_change);

        // Check if calculation exists, and add to points
        if z.is_some() {
            let z = z.unwrap();
            let re = z.re;
            let im = z.im;
            if !re.is_infinite()
                && !re.is_nan()
                && !im.is_infinite()
                && !im.is_nan()
            {
                points.push((z, i as i32, i as i32));
            }
        }
    }
    points
}

pub fn evaulate_points(
    algorithm: Vec<String>,
    x_interval: (i32, i32),
    x_precision: i32,
    y_interval: (i32, i32),
    y_precision: i32,
) -> Vec<(Complex<f32>, i32, i32)> {
    let mut points = vec![];

    // If no z, just evaluate algorithm
    if !algorithm.contains(&"z".to_string()) {
        let z = parse_complex_calculation(algorithm);
        if z.is_some() {
            let z = z.unwrap();
            let re = z.re;
            let im = z.im;
            if !re.is_infinite()
                && !re.is_nan()
                && !im.is_infinite()
                && !im.is_nan()
            {
                points.push((z, 1, 1));
            }
        }
        return points;
    }

    // Loop intervals
    for i in x_interval.0 * x_precision..x_interval.1 * x_precision + 1 {
        let x = i as f32 / x_precision as f32;

        for j in y_interval.0 * y_precision..y_interval.1 * y_precision + 1 {
            let y = j as f32 / y_precision as f32;

            // format complex number in reverse polish notation
            let z = Complex::new(x, y);
            let z = format!("{}+i{}", z.re, z.im);
            let mut z: Vec<String> =
                z.split("+").map(|s| s.to_string()).collect();
            z.push("+".to_string());

            // Create new algorithm with z change
            let mut algorithm_z_change = vec![];
            algorithm.iter().for_each(|v| {
                if v == "z" {
                    algorithm_z_change.append(&mut z.clone());
                } else {
                    algorithm_z_change.push(v.clone());
                }
            });

            // Calculate new algorithm
            let z = parse_complex_calculation(algorithm_z_change);

            // Check if calculation exists, and add to points
            if z.is_some() {
                let z = z.unwrap();
                let re = z.re;
                let im = z.im;
                if !re.is_infinite()
                    && !re.is_nan()
                    && !im.is_infinite()
                    && !im.is_nan()
                {
                    points.push((z, j as i32, i as i32));
                }
            }
        }
    }
    points
}

fn is_string_alphabetic(s: &String) -> bool {
    // for imaginary numbers
    if s == "i" {
        return false;
    }
    for c in s.chars() {
        if !c.is_alphabetic() {
            return false;
        }
    }
    return true;
}

pub fn calculate_with_vars(
    algorithm: Vec<String>,
    variables: &HashMap<String, (i32, Complex<f32>)>,
) -> Option<Complex<f32>> {
    let mut algorithm_var_change = vec![];

    // loop algorithm
    algorithm.iter().for_each(|v| {
        // if is alphabetic, a variable is present
        if is_string_alphabetic(&v) {
            // try to get the variable from hash table
            let z = variables.get(v);

            // if it exists, format and add to algorithm
            if let Some((_, z)) = z {
                let z = format!("{}+i{}", z.re, z.im);
                let mut z: Vec<String> =
                    z.split("+").map(|s| s.to_string()).collect();
                z.push("+".to_string());
                algorithm_var_change.append(&mut z.clone());
            } else {
                algorithm_var_change.push(v.clone());
            }
        } else {
            algorithm_var_change.push(v.clone());
        }
    });
    // parse new algorithm and return it
    let z = parse_complex_calculation(algorithm_var_change);
    z
}
