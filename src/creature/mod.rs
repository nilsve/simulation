use brain::Brain;
use crate::creature::neuron_template::NeuronTemplate;
use crate::NeuronConnectionTemplate;

pub mod neuron;
pub mod neuron_template;
pub mod neuron_connection_template;
pub mod neurons;
mod brain;

pub struct CreatureData {
    pub position: (usize, usize)
}

pub struct Creature<'a> {
    brain: Brain<'a>,
    pub creature_data: CreatureData,
}

impl<'a> Creature<'a> {
    pub fn new(neuron_template: &'a NeuronTemplate, neuron_connection_template: &'a NeuronConnectionTemplate) -> Creature<'a> {
        Creature {
            brain: Brain::new(neuron_template, neuron_connection_template),
            creature_data: CreatureData {
                position: (50, 50)
            }
        }
    }

    pub fn simulate(&mut self) {
        self.brain.simulate(&self.creature_data);

        let neuron_template = self.brain.get_neuron_template();

        for i in 0..neuron_template.output_neurons.len() {
            let output_neuron = neuron_template.output_neurons.get(i).unwrap();
            let neuron_value = *self.brain.get_neuron_values().output_neurons.get(i).expect("Neuron output value not found");

            println!("Neuron value {} for neuron {}", neuron_value, output_neuron.get_name());

            if neuron_value != 0.0 {
                output_neuron.fire(&mut self.creature_data, neuron_value);
            }
        }
    }
}
