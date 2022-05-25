use brain::Brain;
use crate::creature::neuron_template::NeuronTemplate;
use crate::{KillRegion, NeuronConnectionList};

pub mod neuron;
pub mod neuron_template;
pub mod neuron_connection_list;
pub mod neurons;
mod brain;

#[derive(Debug)]
pub struct CreatureData {
    pub position: (usize, usize)
}

pub struct Creature<'a> {
    brain: Brain<'a>,
    pub creature_data: CreatureData,
}

#[derive(Default)]
pub struct SimulationData<'a> {
    pub simulation_step: usize,
    pub total_steps: usize,
    pub map_size: (usize, usize),
    pub kill_regions: &'a [&'a KillRegion],
}

impl<'a> Creature<'a> {
    pub fn from_parents(neuron_template: &'a NeuronTemplate, parent_1: &Creature, parent_2: &Creature) -> Creature<'a> {
        Creature {
            brain: Brain::from_parents(neuron_template, parent_1.brain.get_neuron_connection_list(), parent_2.brain.get_neuron_connection_list()),
            creature_data: CreatureData {
                position: (50, 50)
            }
        }
    }

    pub fn new_random(neuron_template: &'a NeuronTemplate, genome_count: usize) -> Creature<'a> {
        Creature {
            brain: Brain::new(neuron_template, NeuronConnectionList::new(neuron_template, genome_count)),
            creature_data: CreatureData {
                position: (50, 50)
            }
        }
    }

    pub fn simulate(&mut self, simulation_data: SimulationData) {
        self.brain.simulate(&self.creature_data, &simulation_data);

        let neuron_template = self.brain.get_neuron_template();

        for i in 0..neuron_template.output_neurons.len() {
            let output_neuron = neuron_template.output_neurons.get(i).unwrap();
            let neuron_value = *self.brain.get_neuron_values().output_neurons.get(i).expect("Neuron output value not found");

            if neuron_value != 0.0 {
                output_neuron.fire(&mut self.creature_data, &simulation_data, neuron_value);
            }
        }
    }
}
