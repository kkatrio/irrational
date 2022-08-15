use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub mod digits;
pub mod draw;

fn main() {
    let f = File::open("pi1mil.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let len = reader.read_line(&mut line).unwrap();
    println!("read digits: {}", len);

    let digitsstr = line.trim().replacen(".", "", 1);
    println!("digits len after replace dot: {}", digitsstr.len());

    //let pidigits = digits::get_string_digits(&digitsstr.get(..10000).unwrap());
    let pidigits = digits::get_string_digits(&digitsstr);

    let viewbox = draw::ViewBox {
        xmin: 0,
        xmax: 300,
        ymin: 0,
        ymax: 1100,
        xstart: 500.0,
        ystart: 950.0,
    };

    let commands = draw::create_path(&pidigits, &viewbox);
    let document = draw::generate_svg(commands, &viewbox);
    svg::save("image.svg", &document).unwrap();
}
