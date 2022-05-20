use crate::{CreatureData, Neuron, OutputNeuron};

pub struct HorizontalPositionOutputNeuron;

impl Neuron for HorizontalPositionOutputNeuron {
    fn get_name(&self) -> &'static str {
        "HorizontalPositionOutputNeuron"
    }
}

impl OutputNeuron for HorizontalPositionOutputNeuron {
    fn fire(&self, creature_data: &mut CreatureData, neuron_value: f32) {
        if neuron_value < 0.0 {
            creature_data.position.0 -= 1;
        } else {
            creature_data.position.0 += 1;
        }
    }
}

pub struct VerticalPositionOutputNeuron;

impl Neuron for VerticalPositionOutputNeuron {
    fn get_name(&self) -> &'static str {
        "VerticalPositionOutputNeuron"
    }
}

impl OutputNeuron for VerticalPositionOutputNeuron {
    fn fire(&self, creature_data: &mut CreatureData, neuron_value: f32) {
        if neuron_value < 0.0 {
            creature_data.position.1 -= 1;
        } else {
            creature_data.position.1 += 1;
        }
    }
}
