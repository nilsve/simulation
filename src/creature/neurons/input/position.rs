use crate::{CreatureData, InputNeuron, Neuron};

pub struct HorizontalPositionInputNeuron;

impl Neuron for HorizontalPositionInputNeuron {
    fn get_name(&self) -> &'static str {
        "X Position neuron"
    }
}

impl InputNeuron for HorizontalPositionInputNeuron {
    fn get_value(&self, creature_data: &CreatureData) -> f32 {
        creature_data.position.0 as f32
    }
}

pub struct VerticalPositionInputNeuron;

impl Neuron for VerticalPositionInputNeuron {
    fn get_name(&self) -> &'static str {
        "Y Position neuron"
    }
}

impl InputNeuron for VerticalPositionInputNeuron {
    fn get_value(&self, creature_data: &CreatureData) -> f32 {
        creature_data.position.1 as f32
    }
}
