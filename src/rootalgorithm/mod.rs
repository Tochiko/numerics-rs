fn newton_raphson(x: f64, func: fn(x: f64), dfunc: fn(x: f64), acc: f64, maxiter: u32) -> (f64, u32) {
    let mut start = x;
    for i in (0..maxiter) {
        let old = start;
        start = old - func(start)/dfunc(start);
        if (start-old).abs() <= acc {
            return (start, maxiter - i);
        }
    }
    (start, maxiter)
}
