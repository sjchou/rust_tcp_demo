
pub trait Shape {
    fn area(&self) -> f32;
}
struct Circle {
    r: f32,
}
impl Shape for Circle{
    fn area(&self) -> f32{
        3.14 * self.r * self.r
    }
}
struct React {
    width: f32,
    height: f32,
}
impl Shape for React{
    fn area(&self) -> f32{
        self.width * self.height
    }
}

fn printArea<T:Shape>(shape: &T) {
    println!("shape area is:{:?}", shape.area());
}   

fn main() {
    let circle:Circle = Circle{r:1.4};
    let react:React = React{width:1.2, height:2.0};
    printArea(&circle);
    printArea(&react);
}
