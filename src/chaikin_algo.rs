#[derive(Debug)]
pub struct Point {
    pub x: i32,

    pub y: i32,
}

#[derive(Debug)]
pub struct Shape {
    pub lines: Vec<Point>,
}

impl Shape {
    pub fn new(lines: Vec<Point>) -> Shape {
        Shape { lines }
    }

    #[allow(non_snake_case)]
    pub fn chaikin(&self) -> Shape {
        println!("OLD SHAPE: {:?}\n", self.lines);
        let mut new_lines = Vec::new();

        for window in self.lines.windows(2) {
            if let [P1, P2] = window {
                new_lines.push(Point::new(P1.x, P1.y));

                // Qi = 3/4 P[i] + 1/4 P[i+1]
                let qi_x = (3 * P1.x + P2.x) / 4;
                let qi_y = (3 * P1.y + P2.y) / 4;

                new_lines.push(Point::new(qi_x, qi_y));

                // Ri = 1/4 P[i] + 3/4 P[i+1]
                let ri_x = (P1.x + 3 * P2.x) / 4;
                let ri_y = (P1.y + 3 * P2.y) / 4;

                new_lines.push(Point::new(ri_x, ri_y));
            }
        }

        println!("NEW SHAPE: {:?}\n", new_lines);
        Shape::new(new_lines)
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}
