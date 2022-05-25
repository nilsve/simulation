use std::cmp;
use crate::{CreatureData, InputNeuron, Neuron, SimulationData};

pub struct HorizontalWallDistanceInputNeuron;

impl Neuron for HorizontalWallDistanceInputNeuron {
    fn get_name(&self) -> &'static str {
        "Horizontal wall distance neuron"
    }
}

fn closest_distance_to_point(from: usize, point_1: usize, point_2: usize) -> usize {
    let d1 = (from as isize - point_1 as isize).abs() as usize;
    let d2 = (from as isize - point_2 as isize).abs() as usize;
    cmp::min(d1, d2)
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
