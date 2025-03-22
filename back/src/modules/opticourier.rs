use std::sync::{OnceLock, RwLock};

pub struct OptiCourier;

static OPTIMIZED_INSTRUCTIONS: OnceLock<RwLock<Vec<String>>> = OnceLock::new();

impl OptiCourier {
    pub fn receive_raw_instructions(raw_instructions: Vec<String>) {
        let optimized_instructions = OptiCourier::optimize_instructions(raw_instructions.clone());

        let lock = OPTIMIZED_INSTRUCTIONS.get_or_init(|| RwLock::new(vec![]));
        *lock.write().unwrap() = optimized_instructions; // replace data, not clear
    }

    fn optimize_instructions(raw_instructions: Vec<String>) -> Vec<String> {
        let mut optimized_instructions = Vec::new();
        let counterclockwise_symbol = "'";
        let doubletime_symbol = "2";
        for instruction in raw_instructions.clone() {
            let mut optimized_instruction = String::new();
            let mut i = 0;
            while i < instruction.len() {
                println!("i = {}", i);
                if instruction.len() >= i + 8
                    && (&instruction[i + 2..i + 3] == &instruction[i..i + 1]
                        && &instruction[i + 4..i + 5] == &instruction[i..i + 1]
                        && &instruction[i + 6..i + 7] == &instruction[i..i + 1])
                    && (&instruction[i + 1..i + 2] == counterclockwise_symbol
                        && &instruction[i + 3..i + 4] == counterclockwise_symbol
                        && &instruction[i + 5..i + 6] == counterclockwise_symbol
                        && &instruction[i + 7..i + 8] == counterclockwise_symbol)
                {
                    optimized_instruction += "";
                    i += 8;
                } else if instruction.len() >= i + 6
                    && (&instruction[i + 2..i + 3] == &instruction[i..i + 1]
                        && &instruction[i + 4..i + 5] == &instruction[i..i + 1])
                    && (&instruction[i + 1..i + 2] == counterclockwise_symbol
                        && &instruction[i + 3..i + 4] == counterclockwise_symbol
                        && &instruction[i + 5..i + 6] == counterclockwise_symbol)
                {
                    optimized_instruction += &instruction[i..i + 1];
                    i += 6;
                } else if instruction.len() >= i + 4
                    && (&instruction[i + 2..i + 3] == &instruction[i..i + 1])
                    && (&instruction[i + 1..i + 2] == counterclockwise_symbol
                        && &instruction[i + 3..i + 4] == counterclockwise_symbol)
                {
                    optimized_instruction =
                        optimized_instruction + &instruction[i..i + 1] + doubletime_symbol;
                    i += 4;
                } else if instruction.len() >= i + 4
                    && (((&instruction[i + 2..i + 3] == &instruction[i..i + 1])
                        && (&instruction[i + 1..i + 2] == doubletime_symbol
                            && &instruction[i + 3..i + 4] == doubletime_symbol))
                        || (&instruction[i..i + 1] == &instruction[i + 1..i + 2]
                            && &instruction[i + 1..i + 2] == &instruction[i + 2..i + 3]
                            && &instruction[i + 2..i + 3] == &instruction[i + 3..i + 4]))
                {
                    optimized_instruction += "";
                    i += 4;
                } else if instruction.len() >= i + 3
                    && (&instruction[i..i + 1] == &instruction[i + 1..i + 2]
                        && &instruction[i + 1..i + 2] == &instruction[i + 2..i + 3])
                {
                    optimized_instruction =
                        optimized_instruction + &instruction[i..i + 1] + counterclockwise_symbol;
                    i += 3;
                } else if instruction.len() >= i + 2
                    && (&instruction[i..i + 1] == &instruction[i + 1..i + 2])
                {
                    optimized_instruction =
                        optimized_instruction + &instruction[i..i + 1] + doubletime_symbol;
                    i += 2;
                } else {
                    optimized_instruction += &instruction[i..i + 1];
                    i += 1;
                }
            }
            optimized_instructions.push(optimized_instruction);
        }
        optimized_instructions
    }

    pub fn transmit_optimized_instructions() -> Vec<String> {
        let lock = OPTIMIZED_INSTRUCTIONS.get().expect("Data not set");
        let data = lock.read().unwrap();
        data.clone() // return data copy, not move
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimize_instructions() {
        let raw_instructions = vec![String::from("D'R'URU'U'FFFFFFFFFFFFFFFFFFFFFFFB'B2B2DF2F2F'F'")];
        //let raw_instructions = vec![String::from("FFFB'")];
        let actual_optimized_instructions = OptiCourier::optimize_instructions(raw_instructions);
        let expected_optimized_instructions = vec![String::from("D'R'URU2F'B'DF2")];
        assert_eq!(
            actual_optimized_instructions,
            expected_optimized_instructions
        );
    }
}
