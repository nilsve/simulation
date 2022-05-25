use crate::{CreatureData, Neuron, OutputNeuron, SimulationData};

pub struct HorizontalPositionOutputNeuron;

impl Neuron for HorizontalPositionOutputNeuron {
    fn get_name(&self) -> &'static str {
        "HorizontalPositionOutputNeuron"
    }
}

impl OutputNeuron for HorizontalPositionOutputNeuron {
    fn fire(&self, creature_data: &mut CreatureData, simulation_data: &SimulationData, neuron_value: f32) {
        let direction: isize = if neuron_value < 0.0 {
            -1
        } else {
            1
        };

        creature_data.position.0 = (creature_data.position.0 as isize + direction).clamp(0, simulation_data.map_size.0 as isize - 1) as usize;
    }
}

pub struct VerticalPositionOutputNeuron;

impl Neuron for VerticalPositionOutputNeuron {
    fn get_name(&self) -> &'static str {
        "VerticalPositionOutputNeuron"
    }
}

impl OutputNeuron for VerticalPositionOutputNeuron {
    fn fire(&self, creature_data: &mut CreatureData, simulation_data: &SimulationData, neuron_value: f32) {
        let direction: isize = if neuron_value < 0.0 {
            -1
        } else {
            1
        };

        creature_data.position.1 = (creature_data.position.1 as isize + direction).clamp(0, simulation_data.map_size.1 as isize - 1) as usize;
    }
}
