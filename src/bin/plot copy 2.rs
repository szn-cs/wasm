use plotters::prelude::*;
use std::cmp;
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

    let name: &String = &data_files[0];
    let output_filename: String = format!("{name}.png");
    println!("output path: {}", output_filename);

    let mut data_processing: Vec<Vec<(u32, f32)>> = vec![];
    for filename in data_files.iter().skip(1) {
        data_processing.push(get_data_skip(&filename, 0));
        println!("{:?}", filename);
    }
    let mut data_total: Vec<Vec<(u32, f32)>> = vec![];
    for filename in data_files.iter().skip(1) {
        data_total.push(get_data_skip(&filename, 1));
        println!("{:?}", filename);
    }

    let mut max: f32 = 0.0;
    for d in data_total.iter() {
        let m = d.iter().map(|e| e.1).max_by(|a, b| a.total_cmp(b)).unwrap();
        // let m = d.iter().map(|e| e.1).max().unwrap();
        max = max.max(m);
    }

    // let data1 = &data[0];
    // let data2 = &data[1];

    let range_vec = get_range_vec();

    let root = BitMapBackend::new(&output_filename, (1000, 500)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            // "Scaling analysis — Parallel write of Parquet file (30M enteries per thread)",
            "Performance scaling — Parallel write of Parquet file (10M enteries per thread)",
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
            (1.0..max.clone() + 1.0),
        )?;

    chart
        .configure_mesh()
        .x_desc("# of threads")
        .y_desc("Speedup factor")
        .label_style(("sans-serif", 18))
        // .x_label_style(("sans-serif", 10))
        // .y_label_style(("sans-serif", 10))
        .draw()?;

    // chart
    //     .draw_series(PointSeries::of_element(
    //         data_processing[0].clone(),
    //         5,
    //         &GREEN,
    //         &|c, s, st| {
    //             return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
    //                     + Circle::new((0,0),s,st.filled()); // At this point, the new pixel coordinate is established
    //         },
    //     ))
    //     .unwrap()
    //     .label("Processing: Rust native")
    //     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    // chart
    //     .draw_series(PointSeries::of_element(
    //         data_processing[1].clone(),
    //         5,
    //         &MAGENTA,
    //         &|c, s, st| {
    //             return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
    //                     + Circle::new((0,0),s,st.filled()); // At this point, the new pixel coordinate is established
    //         },
    //     ))
    //     .unwrap()
    //     .label("Processing: Wasmer + Wasix")
    //     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &MAGENTA));

    chart
        .draw_series(PointSeries::of_element(
            data_total[0].clone(),
            5,
            &RED,
            &|c, s, st| {
                return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                        + Circle::new((0,0),s,st.filled()); // At this point, the new pixel coordinate is established
            },
        ))
        .unwrap()
        .label("Total: Rust native")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(PointSeries::of_element(
            data_total[1].clone(),
            4,
            &BLUE,
            &|c, s, st| {
                return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                        + Circle::new((0,0),s,st.filled()); // At this point, the new pixel coordinate is established
            },
        ))
        .unwrap()
        .label("Total: Wasmer + Wasix")
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

fn get_data_skip(s: &str, skip: usize) -> Vec<(u32, f32)> {
    let range_vec = get_range_vec();
    let mut data: Vec<(u32, f32)> = vec![];

    if let Ok(mut lines) = read_lines(s) {
        let mut lines = lines.skip(skip).step_by(3);
        let first_line = lines.next().unwrap();
        let value_thread1 = match first_line {
            Ok(s) => s.parse().unwrap(),
            _ => 0.0,
        };
        data.push((range_vec[0], value_thread1 as f32 / value_thread1));

        // Consumes the iterator, returns an (Optional) String
        for line in lines.enumerate() {
            if let (i, Ok(s)) = line {
                let duration: f32 = s.parse().unwrap();
                data.push((range_vec[i] + 1, value_thread1 as f32 / duration));
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
