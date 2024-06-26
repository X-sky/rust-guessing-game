use num::Complex;

/// try to determine if `c` is in the Mandelbrot set, using at most `limit` iterations to decide.
///
/// if `c` is not a member, return `Some(i)`, where `i` is the number of iterations it took for `c` to leave the circle of radius 2.
/// if `c` seems to be a member (more precisely, if we reached the iteration limit without being able to prove that `c` is not a member), return `None`.
pub fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}
