use std::sync::{OnceLock, RwLock};

pub struct OptiCourier;

static OPTIMIZED_INSTRUCTIONS: OnceLock<RwLock<Vec<String>>> = OnceLock::new();

impl OptiCourier {
    pub fn receive_raw_instructions(raw_instructions: Vec<String>) {
        let instructions = OptiCourier::optimize_instructions(raw_instructions.clone());

        let lock = OPTIMIZED_INSTRUCTIONS.get_or_init(|| RwLock::new(vec![]));
        *lock.write().unwrap() = instructions; // replace data, not clear
    }

    fn optimize_instructions(instructions: Vec<String>) -> Vec<String> {
        Vec::new()
    }

    pub fn transmit_optimized_instructions() -> Vec<String> {
        let lock = OPTIMIZED_INSTRUCTIONS.get().expect("Data not set");
        let data = lock.read().unwrap();
        data.clone() // return data copy, not move
    }
}
