use std::fmt::Debug;

// 定义一个 Trait 来表示可以计算面积的类型
trait Area {
    fn calculate_area(&self) -> f64;
}

// 实现圆形的结构体和计算面积的方法
#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn calculate_area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 实现三角形的结构体和计算面积的方法
#[derive(Debug)]
struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Triangle {
    fn calculate_area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 实现正方形的结构体和计算面积的方法
#[derive(Debug)]
struct Square {
    side: f64,
}

impl Area for Square {
    fn calculate_area(&self) -> f64 {
        self.side * self.side
    }
}

// 打印图形的面积
fn print_area<T: Area + Debug>(shape: T) {
    println!("The area is: {:?}", shape.calculate_area());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_area() {
        let circle = Circle { radius: 5.0 };
        let triangle = Triangle { base: 4.0, height: 3.0 };
        let square = Square { side: 6.0 };

        // 使用 assert_eq! 宏来断言打印的结果是否符合预期
        assert_eq!( circle.calculate_area(),  78.53981633974483);
        assert_eq!( triangle.calculate_area(),  6.0 );
        assert_eq!( square.calculate_area(),  36.0);
    }
 
}