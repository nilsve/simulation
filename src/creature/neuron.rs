use crate::creature::CreatureData;

pub trait Neuron {
    fn get_name(&self) -> &'static str;
}

pub trait InputNeuron: Neuron {
    fn get_value(&self, creature_data: &CreatureData) -> f32;
}

pub trait OutputNeuron: Neuron {
    fn fire(&self,  creature_data: &mut CreatureData, neuron_value: f32);
}

#[derive(Clone, PartialOrd, PartialEq, Eq, Ord)]
pub enum NeuronType {
    Input = 0,
    Internal = 1,
    Output = 2,
}

#[derive(Clone)]
pub struct ConnectionIdentifier {
    pub neuron_type: NeuronType,
    pub neuron_index: usize,
}

#[derive(Clone)]
pub struct NeuralConnection {
    pub weight: f32,
    pub from: ConnectionIdentifier,
    pub to: ConnectionIdentifier,
}

