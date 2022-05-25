use crate::{CreatureData, InputNeuron, Neuron, SimulationData};

pub struct SimulationStepInputNeuron;

impl Neuron for SimulationStepInputNeuron {
    fn get_name(&self) -> &'static str {
        "Simulation step neuron"
    }
}

impl InputNeuron for SimulationStepInputNeuron {
    fn get_value(&self, _: &CreatureData, simulation_data: &SimulationData) -> f32 {
        simulation_data.simulation_step as f32 / simulation_data.total_steps as f32
    }
}
