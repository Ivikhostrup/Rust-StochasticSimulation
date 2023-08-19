use plotters::prelude::*;
use crate::monitor::SystemStateSnapshot;

pub fn plot() {
    let root_drawing_area = BitMapBackend::new("images/0.3.png", (600, 400))
        .into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Demo", ("sans-serif", 40))
        .build_cartesian_2d(-10..10, 0..100)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(LineSeries::new((-10..=10).map(|x| (x, x*x)), &GREEN))
        .unwrap()
        .label("Cosine")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    ctx.configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(&WHITE.mix(0.8))  // Optional, for better visibility
        .border_style(&BLACK)               // Optional, for better visibility
        .draw()
        .unwrap();
}
