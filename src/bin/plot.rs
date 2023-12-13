use plotters::prelude::*;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

fn main() -> () {
    let _ = plot();
}

fn plot() -> Result<(), Box<dyn std::error::Error>> {
    let data_files: Vec<String> = env::args().collect();

    let name = &data_files[0];
    let output_filename: String = format!("{name}.png");
    println!("output path: {}", output_filename);

    let mut data: Vec<Vec<(u32, u32)>> = vec![];
    for filename in data_files.iter().skip(1) {
        data.push(get_data(&filename));
        println!("{:?}", filename);
    }
    let data1 = &data[0];
    let data2 = &data[1];

    let range_vec = get_range_vec();

    let root = BitMapBackend::new(&output_filename, (1000, 500)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            // "Scaling analysis — Parallel write of Parquet file (30M enteries per thread)",
            "Scaling analysis — Parallel write of Parquet file (10M enteries per thread)",
            ("sans-serif", 30).into_font(),
        )
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(80)
        // .set_label_area_size(LabelAreaPosition::Left, 50)
        // .set_label_area_size(LabelAreaPosition::Bottom, 50)
        .build_cartesian_2d(
            (range_vec.first().unwrap().clone()..range_vec.last().unwrap().clone())
                .with_key_points(range_vec.into_iter().map(|v| v).collect()),
            (0..data2.first().unwrap().clone().1 + 1000),
        )?;

    chart
        .configure_mesh()
        .x_desc("# of threads")
        .y_desc("Execution duration (ms)")
        .label_style(("sans-serif", 18))
        // .x_label_style(("sans-serif", 10))
        // .y_label_style(("sans-serif", 10))
        .draw()?;

    chart
        .draw_series(PointSeries::of_element(
            data[0].clone(),
            5,
            &RED,
            &|c, s, st| {
                return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                        + Circle::new((0,0),s,st.filled()); // At this point, the new pixel coordinate is established
            },
        ))
        .unwrap()
        .label("Rust native")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(PointSeries::of_element(
            data[1].clone(),
            4,
            &BLUE,
            &|c, s, st| {
                return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                        + Circle::new((0,0),s,st.filled()); // At this point, the new pixel coordinate is established
            },
        ))
        .unwrap()
        .label("Wasmer + Wasix")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .position(SeriesLabelPosition::UpperRight)
        .draw()
        .unwrap();

    root.present()?;

    Ok(())
}

fn get_range_vec() -> Vec<u32> {
    let mut range_vec = vec![];

    // calculate range values
    for t in 1..=40 {
        // println!("{}", 2_u32.pow(n));
        range_vec.push(t);
    }

    range_vec
}

fn get_data(s: &str) -> Vec<(u32, u32)> {
    let range_vec = get_range_vec();
    let mut data = vec![];

    if let Ok(lines) = read_lines(s) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.step_by(2).enumerate() {
            if let (i, Ok(s)) = line {
                let duration: u32 = s.parse().unwrap();
                data.push((range_vec[i], duration));
                // data.push((range_vec[i], duration));
                println!("data: {} @ line {}", duration, i);
            }
        }
    };

    data
}

// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
