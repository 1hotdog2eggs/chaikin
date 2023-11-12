pub mod chaikin_algo {

    pub struct Point {
        x: u32,
        y: u32,
    }
    //use public functions here
    pub struct Shape {
        pub lines: Vec<(Point, Point)>,
    }

    impl Shape {
        pub fn ChaikinAlgo(&self) -> Shape {
            //execute logic here
        }
    }
}
