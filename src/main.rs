use macroquad::prelude::*;
use crate::creature::{Creature, CreatureData, neurons};
use crate::creature::neuron::{InputNeuron, Neuron, OutputNeuron};
use crate::creature::neuron_connection_template::NeuronConnectionTemplate;
use crate::creature::neuron_template::NeuronTemplate;

mod creature;

#[macroquad::main("Simulation")]
async fn main() {
    let input_neurons: Vec<Box<dyn InputNeuron>> = vec![
        Box::new(neurons::input::position::HorizontalPositionInputNeuron {}),
        Box::new(neurons::input::position::VerticalPositionInputNeuron {})
    ];

    let output_neurons: Vec<Box<dyn OutputNeuron>> = vec![
        Box::new(neurons::output::position::HorizontalPositionOutputNeuron {}),
        Box::new(neurons::output::position::VerticalPositionOutputNeuron {})
    ];

    let neuron_template = NeuronTemplate::new(4, input_neurons, output_neurons);
    let neuron_connection_template = NeuronConnectionTemplate::new(&neuron_template, 24);
    let mut creature = Creature::new(&neuron_template, &neuron_connection_template);

    for _ in 0..300 {
        creature.simulate();

        clear_background(WHITE);

        draw_rectangle(creature.creature_data.position.0 as f32 * 10.0, creature.creature_data.position.1 as f32 * 10.0, 10.0, 10.0, BLACK);

        next_frame().await
    }
}
