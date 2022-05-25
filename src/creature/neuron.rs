use std::any::Any;
use std::hash::{Hash, Hasher};
use crate::creature::CreatureData;
use crate::SimulationData;

pub trait Neuron {
    fn get_name(&self) -> &'static str;
}

pub trait InputNeuron: Neuron {
    fn get_value(&self, creature_data: &CreatureData, simulation_data: &SimulationData) -> f32;
}

pub trait OutputNeuron: Neuron {
    fn fire(&self,  creature_data: &mut CreatureData, simulation_data: &SimulationData, neuron_value: f32);
}

#[repr(u8)]
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

impl NeuralConnection {
    // pub fn compare(&self, other: &NeuralConnection) -> f32 {
    //     let set_bits = self.get_set_bits();
    //     let set_bits_other = other.get_set_bits();
    //
    //     let weight = self.weight;
    //     let weight_other = other.weight;
    //
    //     const MAX_BITS: u8 = 10;
    //
    //     let similar_bits = MAX_BITS - set_bits.abs_diff(set_bits_other);
    // }
    //
    // fn get_set_bits(&self) -> u8 {
    //     self.from.get_set_bits() + self.to.get_set_bits()
    // }
}

// #[test]
// fn test_get_serialized() {
//     let connection = NeuralConnection {
//         weight: 1.0,
//         from: ConnectionIdentifier {
//             neuron_index: 0,
//             neuron_type: NeuronType::Internal
//         },
//         to: ConnectionIdentifier {
//             neuron_index: 0,
//             neuron_type: NeuronType::Output
//         }
//     };
//
//     let serialized = connection.serialize();
//
//     assert_eq!(serialized, 0b1000_0000_0000_0000_0000_0000_0000_0000);
// }
