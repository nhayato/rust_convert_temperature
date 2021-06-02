fn main() {
    let fahrenhet = c2f(40.0 as f64);
    println!("40°C is: {} [°F]", fahrenhet);
    let celsius = f2c(100.0);
    println!("100°F is: {} [°C]", celsius);
}

fn c2f(celsius: f64) -> f64 {
    (9.0 / 5.0) * celsius + 32.0
}

fn f2c(fahrenhet: f64) -> f64 {
    5.0 / 9.0 * (fahrenhet - 32.0)
}
