fn areasquare<T: std::ops::Mul<Output = T> + Copy>(r: T) -> T {
    r*r
}

fn main() {
    println!("{}", areasquare(3));
}