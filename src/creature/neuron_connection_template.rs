use std::ops::Range;
use rand::{Rng, thread_rng};
use crate::NeuronTemplate;
use crate::creature::neuron::{ConnectionIdentifier, NeuralConnection, NeuronType};

// The genomes
pub struct NeuronConnectionTemplate {
    connection_list: Vec<NeuralConnection>,
}

impl NeuronConnectionTemplate {
    // Creates a random neuron connection template based on a neuron template
    pub fn new(neuron_template: &NeuronTemplate, genome_count: usize) -> NeuronConnectionTemplate {
        let mut connection_list: Vec<NeuralConnection> = (0..genome_count).map(|_| {
            let input_type = match thread_rng().gen_range::<usize, Range<usize>>(0..2) {
                0 => NeuronType::Input,
                _ => NeuronType::Internal,
            };

            let output_type = match thread_rng().gen_range::<usize, Range<usize>>(0..2) {
                0 => NeuronType::Internal,
                _ => NeuronType::Output,
            };

            NeuralConnection {
                weight: thread_rng().gen_range::<f32, Range<f32>>(-4.0..4.0),
                from: ConnectionIdentifier {
                    neuron_index: NeuronConnectionTemplate::get_random_neuron_index(neuron_template, &input_type),
                    neuron_type: input_type,
                },
                to: ConnectionIdentifier {
                    neuron_index: NeuronConnectionTemplate::get_random_neuron_index(neuron_template, &output_type),
                    neuron_type: output_type,
                },
            }
        }).collect();

        connection_list.sort_by(|first, second| {
            first.to.neuron_type.cmp(&second.to.neuron_type)
        });

        NeuronConnectionTemplate {
            connection_list
        }
    }

    pub fn get_connection_list(&self) -> &Vec<NeuralConnection> {
        &self.connection_list
    }

    fn get_random_neuron_index(neuron_template: &NeuronTemplate, neuron_type: &NeuronType) -> usize {
        let size = match neuron_type {
            NeuronType::Input => neuron_template.input_neurons.len(),
            NeuronType::Internal => neuron_template.internal_neurons.len(),
            NeuronType::Output => neuron_template.output_neurons.len(),
        };

        thread_rng().gen_range(0..size)
    }
}
