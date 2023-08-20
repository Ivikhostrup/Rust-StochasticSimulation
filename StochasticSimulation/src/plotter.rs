use std::collections::HashSet;
use plotters::prelude::*;
use crate::monitor::SystemStateSnapshot;

pub fn plot(data: &Vec<SystemStateSnapshot>) {
    let root_drawing_area = BitMapBackend::new("images/0.3.png", (600, 400))
        .into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let min_time = data.first().map(|snap| snap.time).unwrap();
    let max_time = data.last().map(|snap| snap.time). unwrap();

    // Flatten the list of reactions from each snapshot into a continuous sequence
    // of all reactions across all snapshots, and then find the maximum reaction.
    let max_quantity = data.iter()
        .flat_map(|snap| snap.reactions.iter())
        .max()
        .unwrap();

    let mut ctx = ChartBuilder::on(&root_drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Demo", ("sans-serif", 40))
        .build_cartesian_2d(min_time..max_time, 0..max_quantity)
        .unwrap();

    let first_snapshot = data.first().unwrap();
    let unique_species = first_snapshot.reactions.iter()
        .flat_map(|reaction| {
            let reaction_guard = reaction.lock().unwrap();

            reaction_guard.reactants.iter().chain(reaction_guard.products.iter())
        })
        .map(|species| {
            let species_guard = species.lock().unwrap();
            species_guard.name.clone()
    }).collect::<Vec<String>>();

    fn generate_color(index: usize) -> RGBColor {
        let colors = [RED, GREEN, BLUE, MAGENTA, CYAN, YELLOW];  // Add more colors if necessary
        colors[index % colors.len()]
    }

    for (index, species_name) in &unique_species.iter().enumerate() {
        let color = generate_color(index);

        let series_data: Vec<_> = data.iter().map(|snap| {
            let time = snap.time;

            let quantity_for_species = snap.reactions.iter()
                .flat_map(|reaction| {
                    let reaction_guard = reaction.lock().unwrap();

                    reaction_guard.reactants.iter().chain(reaction_guard.products.iter())
                })
                .find(|species| {
                    let species_guard = species.lock().unwrap();

                    &species_guard.name == species_name
            }).unwrap();

            let species_guard = quantity_for_species.lock().unwrap();
            let quantity = species_guard.get_quantity();

            (time, quantity)
        }).collect();

        ctx.draw_series(LineSeries::new(series_data, &color))
            .unwrap()
            .label(species_name)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color));
    }

    ctx.configure_mesh().draw().unwrap();

    ctx.configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();
}
