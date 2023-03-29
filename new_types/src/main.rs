struct Radians(f32);

struct Degrees(f32);

impl Degrees {
    fn to_radians(&self) -> f32 {
        self.0.to_radians()
    }
}

// Rust's `from` trait, which automatically sets up `into()` for types.
impl From<Degrees> for Radians {
    fn from(degrees: Degrees) -> Radians {
        Radians(degrees.0.to_radians())
    }
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;
    fn add(mut self, rhs: Point) -> Point {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

// Take a starting point, project a point at `angle` degree over `distance` and return the final coordinates.
/*fn project_angle(start: (f32, f32), angle: Radians, radius: f32) -> (f32, f32) {
    (
        0.0 - (start.0 + radius * f32::sin(angle.0)),
        start.1 + radius * f32::cos(angle.0),
    )
}*/
// Done with generics. Generic functions take one or more args as a trait and only work for variables that implement that trait.
// `<A: TRAIT>` defines parameter type A to have to match the listed trait.
// Into trait automatically generates when you create a `From` trace (but not vice versa).
// We require that `Into<Radians>` exists for whatever type is sent as type A.
fn project_angle<A: Into<Radians>>(start: Point, angle: A, radius: f32) -> (f32, f32) {
    let angle: Radians = angle.into();
    (
        0.0 - (start.x + radius * f32::sin(angle.0)),
        start.y + radius * f32::cos(angle.0),
    )
}

fn main() {
    let start = Point::new(0.0, 0.0);
    let mut start = start + Point::new(1.0, 1.0);
    start += Point::new(1.0, 1.0);
    // let finish = project_angle(start, Degrees(180.0).into(), 10.0);
    let finish = project_angle(start, Degrees(180.0), 10.0);
    println!("{finish:?}");
}
