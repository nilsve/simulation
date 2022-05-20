use crate::creature::neuron::{InputNeuron, OutputNeuron};

pub struct NeuronTemplate {
    pub input_neurons: Vec<Box<dyn InputNeuron>>,
    pub internal_neurons: Vec<f32>, // Starting value of neuron
    pub output_neurons: Vec<Box<dyn OutputNeuron>>,
}

impl NeuronTemplate {
    pub fn new(internal_neuron_count: usize, input_neurons: Vec<Box<dyn InputNeuron>>, output_neurons: Vec<Box<dyn OutputNeuron>>) -> NeuronTemplate {
        assert!(input_neurons.len() > 0);
        assert!(output_neurons.len() > 0);

        NeuronTemplate {
            internal_neurons: vec![0.5; internal_neuron_count],
            input_neurons,
            output_neurons,
        }
    }
}
