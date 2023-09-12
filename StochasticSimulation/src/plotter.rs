use std::collections::{HashMap, HashSet};
use plotters::prelude::*;
use crate::monitor::SystemStateSnapshot;
use crate::reaction::SpeciesRole;

pub fn plot(data: &Vec<SystemStateSnapshot>, species_to_plot: &[(&str, SpeciesRole)]) {
    let root_drawing_area = BitMapBackend::new("images/0.4.png", (600, 400))
        .into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let min_time = data.first().map(|snap| snap.time).unwrap();
    let max_time = data.last().map(|snap| snap.time). unwrap();

    // Flatten the list of reactions from each snapshot into a continuous sequence
    // of all reactions across all snapshots, and then find the maximum reaction.
    let max_quantity = 100;
        //data.iter()
        //.flat_map(|snap| snap.reactions.iter())
        //.max()
        //.unwrap();

    let mut ctx = ChartBuilder::on(&root_drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Demo", ("sans-serif", 40))
        .build_cartesian_2d(min_time..100.0, 0..max_quantity)
        .unwrap();

    let species_to_plot_names: HashSet<&str> = species_to_plot.iter()
        .map(|(name, _role) | *name)
        .collect();

    let mut series_data_by_species: HashMap<String, Vec<(f64, i32)>> = HashMap::new();

    for snapshots in data.iter() {
        let time = snapshots.time;

        for reaction in snapshots.reactions.iter() {
            let reaction_guard = reaction.lock().unwrap();

            for species in reaction_guard.reactants.iter().chain(reaction_guard.products.iter()) {
                let species_guard = species.lock().unwrap();

                if species_to_plot_names.contains(&species_guard.name.as_str()) {
                    series_data_by_species
                        .entry(species_guard.name.clone())
                        .or_default()
                        .push((time, species_guard.get_quantity()))
                }
            }
        }
    }

    fn generate_color(index: usize) -> RGBColor {
        let colors = [RED, GREEN, BLUE, MAGENTA, CYAN, YELLOW];  // Add more colors if necessary
        colors[index % colors.len()]
    }

    for (index, (species_name, species_data)) in series_data_by_species.iter().enumerate() {
        let color = generate_color(index);

        ctx.draw_series(LineSeries::new(
            species_data.iter().cloned(),
            &color
        )).unwrap()
            .label(species_name)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color));
    }

    ctx.configure_mesh().draw().unwrap();

    ctx.configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();
}
