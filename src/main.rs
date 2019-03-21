extern crate csv;

use std::path::Path;
use std::fs::File;
use std::error::Error;

use clap::{Arg, App};

use gnuplot::{Figure, Caption, Color};

struct Point {
    x: f32,
    y: u32,
}

fn parse_csv(path: &Path, points: &mut Vec<Point>) -> Result<(), Box<Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        points.push(Point{ x: record[0].parse()?, y: record[1].parse()? })
    }
    Ok(())
}

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("gnuplot script")
        .arg(Arg::with_name("file")
             .short("f")
             .long("file")
             .value_name("FILE")
             .help("csv file path")
             .takes_value(true))
        .arg(Arg::with_name("title")
             .short("t")
             .long("title")
             .value_name("TITLE")
             .help("graph title"))
        .get_matches();

    let csv_path = match matches.value_of("file") {
        Some(path) => path,
        None => panic!("specify csv file")
    };

    let csv_path = Path::new(csv_path);
    if !csv_path.exists() {
        panic!("No such or directory: {}", csv_path.display());
    }

    let caption = match matches.value_of("title") {
        Some(title) => title,
        None => "no title"
    };

    let mut points = Vec::<Point>::new();
    parse_csv(&csv_path, &mut points).expect("fail");

    let mut x = Vec::<f32>::new();
    let mut y = Vec::<u32>::new();

    for point in points {
        x.push(point.x);
        y.push(point.y);
    }

    let mut fg = Figure::new();

    fg.axes2d()
        .lines(&x, &y, &[Caption(caption), Color("black")]);

    fg.show();
}
