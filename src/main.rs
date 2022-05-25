extern crate core;

use std::ops::Range;
use rand::{Rng, thread_rng};
use crate::creature::{Creature, CreatureData, neurons, SimulationData};
use crate::creature::neuron::{InputNeuron, Neuron, OutputNeuron};
use crate::creature::neuron_connection_list::NeuronConnectionList;
use crate::creature::neuron_template::NeuronTemplate;
use crate::renderer::render_screen;

mod creature;
mod renderer;

const CREATURE_COUNT: i32 = 200;
const STEP_COUNT: usize = 100;

#[macroquad::main("Simulation")]
async fn main() {
    let input_neurons: Vec<Box<dyn InputNeuron>> = vec![
        Box::new(neurons::input::position::HorizontalPositionInputNeuron {}),
        Box::new(neurons::input::position::VerticalPositionInputNeuron {}),
        Box::new(neurons::input::simulation_step::SimulationStepInputNeuron {}),
        Box::new(neurons::input::distance::HorizontalWallDistanceInputNeuron {}),
        Box::new(neurons::input::distance::VerticalWallDistanceInputNeuron {}),
    ];

    let output_neurons: Vec<Box<dyn OutputNeuron>> = vec![
        Box::new(neurons::output::position::HorizontalPositionOutputNeuron {}),
        Box::new(neurons::output::position::VerticalPositionOutputNeuron {})
    ];

    let neuron_template = NeuronTemplate::new(4, input_neurons, output_neurons);

    let mut creatures = Vec::new();

    for _ in 0..CREATURE_COUNT {
        creatures.push(Creature::new_random(&neuron_template, 24));
    }

    let mut generation = 0;
    loop {
        for simulation_step in 0..STEP_COUNT {
            creatures.iter_mut().for_each(|creature| creature.simulate(SimulationData { simulation_step, total_steps: STEP_COUNT, map_size: (100, 100) }));

            render_screen(creatures.as_slice()).await;
        }

        let survived: Vec<&Creature> = creatures.iter().filter(|creature| {
            creature.creature_data.position.0 > 50 && creature.creature_data.position.1 < 50
        }).collect();

        creatures = survived.iter().map(|creature| {
            Creature::from_parents(&neuron_template, creature, survived.get(thread_rng().gen_range::<usize, Range<usize>>(0..survived.len())).unwrap())
        }).collect();

        generation += 1;

        println!("Generation {}", generation);
    }
}
