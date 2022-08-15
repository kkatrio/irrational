use svg::node::element::path::{Command, Data, Position};
use svg::node::element::Path;
use svg::Document;

#[derive(Debug)]
struct Direction {
    sines: Vec<f64>,
    cosines: Vec<f64>,
}

impl Direction {
    fn new() -> Direction {
        let sines = (0..10)
            .map(|i| (i as f64 * 36.0).to_radians().sin())
            .collect();
        let cosines = (0..10)
            .map(|i| (i as f64 * 36.0).to_radians().cos())
            .collect();
        Direction { sines, cosines }
    }
}

struct Turtle {
    x: f64,
    y: f64,
    r: f64,
    direction: Direction,
}

impl Turtle {
    fn turn(&mut self, bearing: u32) {
        self.x += &self.r * &self.direction.sines[bearing as usize];
        self.y += &self.r * &self.direction.cosines[bearing as usize];
    }
}

pub struct ViewBox {
    pub xmin: usize,
    pub ymin: usize,
    pub xmax: usize,
    pub ymax: usize,
    pub xstart: f64,
    pub ystart: f64,
}

pub fn create_path(operations: &Vec<u32>, canvas: &ViewBox) -> Vec<Command> {
    let mut commands = Vec::with_capacity(operations.len());
    let directions = Direction::new();
    let mut turtle = Turtle {
        x: canvas.xstart,
        y: canvas.ystart,
        direction: directions,
        r: 1.0,
    };

    // draw an outline of the viewbox
    /*
    commands.push(Command::Move(
        Position::Absolute,
        (canvas.xmin, canvas.ymin).into(),
    ));
    let boarder = Command::Line(Position::Absolute, (canvas.xmin, canvas.ymax).into());
    commands.push(boarder);
    let boarder = Command::Line(Position::Absolute, (canvas.xmax, canvas.ymax).into());
    commands.push(boarder);
    let boarder = Command::Line(Position::Absolute, (canvas.xmax, canvas.ymin).into());
    commands.push(boarder);
    let boarder = Command::Line(Position::Absolute, (canvas.xmin, canvas.ymin).into());
    commands.push(boarder);
    */

    // move to starting position
    commands.push(Command::Move(
        Position::Absolute,
        (turtle.x, turtle.y).into(),
    ));

    for op in operations {
        turtle.turn(*op);
        let line = Command::Line(Position::Absolute, (turtle.x, turtle.y).into());
        commands.push(line);
    }
    commands
}

pub fn generate_svg(commands: Vec<Command>, canvas: &ViewBox) -> Document {
    let data = Data::from(commands);

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 0.1)
        .set("d", data);

    Document::new()
        .set(
            "viewBox",
            (canvas.xmin, canvas.ymin, canvas.xmax, canvas.ymax),
        )
        .add(path)
}
