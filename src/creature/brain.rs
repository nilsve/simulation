use crate::{CreatureData, NeuronConnectionList, NeuronTemplate, SimulationData};
use crate::creature::neuron::{NeuronType};

pub struct NeuronValues {
    pub internal_neurons: Vec<f32>,
    pub output_neurons: Vec<f32>,
}

impl NeuronValues {
    fn new(neuron_template: &NeuronTemplate) -> NeuronValues {
        NeuronValues {
            internal_neurons: neuron_template.internal_neurons.clone(),
            output_neurons: vec![0.0; neuron_template.output_neurons.len()]
        }
    }
}

pub struct Brain<'a> {
    neuron_template: &'a NeuronTemplate,
    neuron_connection_list: NeuronConnectionList,
    neuron_values: NeuronValues,
}

impl<'a> Brain<'a> {
    pub fn new(neuron_template: &'a NeuronTemplate, neuron_connection_list: NeuronConnectionList) -> Brain<'a> {
        Brain {
            neuron_template,
            neuron_connection_list,
            neuron_values: NeuronValues::new(neuron_template),
        }
    }

    pub fn from_parents(neuron_template: &'a NeuronTemplate, parent_1: &NeuronConnectionList, parent_2: &NeuronConnectionList) -> Brain<'a> {
        Brain {
            neuron_template,
            neuron_connection_list: NeuronConnectionList::from_parents(parent_1, parent_2),
            neuron_values: NeuronValues::new(neuron_template),
        }
    }

    pub fn get_neuron_template(&self) -> &NeuronTemplate {
        self.neuron_template
    }

    pub fn get_neuron_values(&self) -> &NeuronValues {
        &self.neuron_values
    }

    pub fn get_neuron_connection_list(&self) -> &NeuronConnectionList {
        &self.neuron_connection_list
    }

    pub fn simulate(&mut self, creature_data: &CreatureData, simulation_data: &SimulationData) {
        let mut internal_neuron_accumulator: Vec<f32> = vec![0.0; self.neuron_template.internal_neurons.len()];
        let mut output_neuron_accumulator: Vec<f32> = vec![0.0; self.neuron_template.output_neurons.len()];

        let mut neuron_outputs_computed = false;
        self.neuron_connection_list.get_connection_list().iter().for_each(|connection| {
            if connection.to.neuron_type == NeuronType::Output && !neuron_outputs_computed {
                // All input & internal neurons are processed, store their data for next simulation run

                for i in 0..self.neuron_values.internal_neurons.len() {
                    *self.neuron_values.internal_neurons.get_mut(i).unwrap() =
                        internal_neuron_accumulator.get(i).expect("Internal neuron accumulator item not found").tanh();
                }

                neuron_outputs_computed = true;
            }

            let neuron_value = match connection.from.neuron_type {
                NeuronType::Input => {
                    self
                        .neuron_template
                        .input_neurons
                        .get(connection.from.neuron_index)
                        .expect("Input neuron not found in template")
                        .get_value(&creature_data, &simulation_data)
                },
                NeuronType::Internal => {
                    let value = *self.neuron_values.internal_neurons.get(connection.from.neuron_index).expect("Internal neuron not found in brain values");
                    assert!(value.abs() <= 1.0);
                    value
                }
                NeuronType::Output => {
                    *self.neuron_values.output_neurons.get(connection.from.neuron_index).expect("Output neuron not found in brain values")
                }
            };

            let output = match connection.to.neuron_type {
                NeuronType::Input => { panic!("Output neuron was set to an input neuron") }
                NeuronType::Internal => { internal_neuron_accumulator.get_mut(connection.to.neuron_index).expect("Internal neuron accumulator not found") }
                NeuronType::Output => { output_neuron_accumulator.get_mut(connection.to.neuron_index).expect("Output neuron accumulator not found")}
            };

            *output += neuron_value * connection.weight;
        });

        output_neuron_accumulator.iter_mut().for_each(|val| *val = val.tanh());

        for i in 0..self.neuron_values.output_neurons.len() {
            *self.neuron_values.output_neurons.get_mut(i).unwrap() =
                *output_neuron_accumulator.get(i).expect("Output neuron accumulator not found");
        }
    }
}
