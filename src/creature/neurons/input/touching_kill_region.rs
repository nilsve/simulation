use crate::{CreatureData, InputNeuron, Neuron, SimulationData};
use crate::neurons::input::closest_distance_to_point;

pub struct TouchingKillRegionNeuron;

impl Neuron for TouchingKillRegionNeuron {
    fn get_name(&self) -> &'static str {
        "Is touching a kill region"
    }
}

impl InputNeuron for TouchingKillRegionNeuron {
    fn get_value(&self, creature_data: &CreatureData, simulation_data: &SimulationData) -> f32 {
        let touching = simulation_data.kill_regions.iter().any(|region| {
            region.is_in_region(&creature_data.position)
        });

        match touching {
            true => 1.0,
            false => 0.0,
        }
    }
}
