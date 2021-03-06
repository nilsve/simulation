extern crate core;

use std::ops::Range;
use macroquad::input::{is_mouse_button_down, mouse_position_local, MouseButton};
use macroquad::prelude::mouse_position;
use rand::{Rng, thread_rng};
use crate::creature::{Creature, CreatureData, neurons, SimulationData};
use crate::creature::neuron::{InputNeuron, Neuron, OutputNeuron};
use crate::creature::neuron_connection_list::NeuronConnectionList;
use crate::creature::neuron_template::NeuronTemplate;
use crate::game::KillRegion;
use crate::renderer::render_screen;

mod creature;
mod renderer;
mod game;

const CREATURE_COUNT: i32 = 400;
const STEP_COUNT: usize = 100;
const GENOME_COUNT: usize = 24;
const NEURON_COUNT: usize = 4;

#[macroquad::main("Simulation")]
async fn main() {
    let input_neurons: Vec<Box<dyn InputNeuron>> = vec![
        Box::new(neurons::input::position::HorizontalPositionInputNeuron {}),
        Box::new(neurons::input::position::VerticalPositionInputNeuron {}),
        Box::new(neurons::input::simulation_step::SimulationStepInputNeuron {}),
        Box::new(neurons::input::distance::HorizontalWallDistanceInputNeuron {}),
        Box::new(neurons::input::distance::VerticalWallDistanceInputNeuron {}),
        Box::new(neurons::input::touching_kill_region::TouchingKillRegionNeuron {}),
    ];

    let output_neurons: Vec<Box<dyn OutputNeuron>> = vec![
        Box::new(neurons::output::position::HorizontalPositionOutputNeuron {}),
        Box::new(neurons::output::position::VerticalPositionOutputNeuron {})
    ];

    let neuron_template = NeuronTemplate::new(NEURON_COUNT, input_neurons, output_neurons);

    let mut creatures = initialise_creatures(&neuron_template);

    let mut moving_region = KillRegion {
        top_left: (0,0),
        bottom_right: (10,10),
    };

    let mut generation = 0;
    loop {

        let kill_regions: [&KillRegion; 6] = [
            &KillRegion { // Left side of screen
                top_left: (0, 0),
                bottom_right: (10, 100),
            },
            &KillRegion { // Right side of screen
                top_left: (90, 0),
                bottom_right: (100, 100),
            },
            &KillRegion { // Top side of screen
                top_left: (0, 0),
                bottom_right: (100, 10),
            },
            &KillRegion { // Bottom side of screen
                top_left: (10, 90),
                bottom_right: (90, 100),
            },
            &KillRegion { // Center of screen
                top_left: (40, 40),
                bottom_right: (60, 60),
            },
            &moving_region
        ];

        for simulation_step in 0..STEP_COUNT {
            creatures.iter_mut().for_each(|creature| {
                let simulation_data = SimulationData {
                    simulation_step,
                    total_steps: STEP_COUNT,
                    map_size: (100, 100),
                    kill_regions: &kill_regions,
                };

                creature.simulate(simulation_data)
            });

            render_screen(creatures.as_slice(), &kill_regions).await;
        }

        let mut survived: Vec<&Creature> = creatures.iter().filter(|creature| {
            !kill_regions.iter().any(|region| region.is_in_region(&creature.creature_data.position))
        }).collect();

        if (survived.is_empty()) {
            println!("No survivors! Stopping simulation :(");
            return;
        }

        creatures = (0..CREATURE_COUNT).map(|_| {
            Creature::from_parents(
                &neuron_template,
                survived.get(thread_rng().gen_range::<usize, Range<usize>>(0..survived.len())).unwrap(),
                survived.get(thread_rng().gen_range::<usize, Range<usize>>(0..survived.len())).unwrap()
            )
        }).collect();

        generation += 1;


        if is_mouse_button_down(MouseButton::Left) {
            let pos = mouse_position();

            let width = moving_region.bottom_right.0 - moving_region.top_left.0;
            let height = moving_region.bottom_right.1 - moving_region.top_left.1;

            moving_region.top_left.0 = pos.0 as usize / 10;
            moving_region.top_left.1 = pos.1 as usize / 10;

            moving_region.bottom_right.0 = pos.0 as usize / 10 + width;
            moving_region.bottom_right.1 = pos.1 as usize / 10 + height;

            println!("set moving region {:?}", moving_region);
        }

        println!("Generation {}", generation);
    }
}

fn initialise_creatures(neuron_template: &NeuronTemplate) -> Vec<Creature> {
    (0..CREATURE_COUNT).map(|_| {
        Creature::new_random(&neuron_template, GENOME_COUNT)
    }).collect()
}
