use crate::{CreatureData, InputNeuron, Neuron, SimulationData};
use crate::neurons::input::closest_distance_to_point;

pub struct HorizontalWallDistanceInputNeuron;

impl Neuron for HorizontalWallDistanceInputNeuron {
    fn get_name(&self) -> &'static str {
        "Horizontal wall distance neuron"
    }
}

impl InputNeuron for HorizontalWallDistanceInputNeuron {
    fn get_value(&self, creature_data: &CreatureData, simulation_data: &SimulationData) -> f32 {
        closest_distance_to_point(creature_data.position.0, 0, simulation_data.map_size.0) as f32 / simulation_data.map_size.0 as f32
    }
}

pub struct VerticalWallDistanceInputNeuron;

impl Neuron for VerticalWallDistanceInputNeuron {
    fn get_name(&self) -> &'static str {
        "Vertical wall distance neuron"
    }
}

impl InputNeuron for VerticalWallDistanceInputNeuron {
    fn get_value(&self, creature_data: &CreatureData, simulation_data: &SimulationData) -> f32 {
        closest_distance_to_point(creature_data.position.1, 0, simulation_data.map_size.1) as f32 / simulation_data.map_size.1 as f32
    }
}
