use plotters::chart::ChartBuilder;
use plotters::prelude::{BitMapBackend, Color, IntoDrawingArea, IntoFont, Palette, Rectangle, SeriesLabelPosition, WHITE};
use plotters::series::LineSeries;
use plotters::style::{BLACK, Palette99};

pub fn draw_chart<T>(data: Vec<Vec<T>>,names: Vec<&str>, n_range: impl Iterator<Item=usize> + Clone, name: &str, scale: impl Fn(f64, f64) -> f64)
    where T: Clone + PartialOrd, f64: From<T> {
    let file = format!("charts/chart_{}.png", name);

    let mut n_range_copy = n_range.clone();
    let first = n_range_copy.next().unwrap() as f64;
    let last = n_range_copy.last().unwrap() as f64;
    let x_range = first..last;

    let max  = data.iter()
        .map(|y| n_range.clone().zip(y.iter())
            .map(|(x,y)| (x as f64 , f64::from(y.clone())))
            .fold(0.0,|a, (bx,by) | {
                let b = scale(bx, by);
                if a > b { a } else { b }
            }))
        .reduce(|a, b| if a > b { a } else { b }).unwrap();

    let y_range = 0.0..(max * 1.1);


    let drawing_area = BitMapBackend::new(&file, (1280, 720)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&drawing_area)
        .margin(5)
        .caption(name, ("Calibri", 40).into_font())
        .set_all_label_area_size(40)
        .build_cartesian_2d(x_range, y_range)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();


    for i in data.into_iter().zip(names).enumerate() {
        let (num, (vals, name)) = i;
        ctx.draw_series(LineSeries::new(
            n_range.clone().zip(vals.into_iter())
                .map(|(x, y)| (x as f64, scale(x as f64, f64::from(y)))), Palette99::pick(num))).unwrap()
            .label(name).legend(move |(x, y)| Rectangle::new([(x, y - 8), (x + 15, y + 7)], Palette99::pick(num).filled()));
    }

    ctx.configure_series_labels().border_style(&BLACK).label_font(("Calibri", 20)).position(SeriesLabelPosition::UpperLeft).background_style(&WHITE).draw().unwrap();
    drawing_area.present().expect("Failed to save chart");
}