use std::sync::{OnceLock, RwLock};

pub struct OptiCourier;

static OPTIMIZED_INSTRUCTIONS: OnceLock<RwLock<Vec<String>>> = OnceLock::new();

impl OptiCourier {
    pub fn receive_raw_instruction(raw_instruction: String) {
        let optimized_instruction = OptiCourier::optimize_instructions(raw_instruction.clone());
        let mut optimized_instructions = OptiCourier::transmit_optimized_instructions();
        optimized_instructions.push(optimized_instruction);
        let lock = OPTIMIZED_INSTRUCTIONS.get_or_init(|| RwLock::new(vec![]));
        *lock.write().unwrap() = optimized_instructions; // replace data, not clear
    }

    fn optimize_instructions(mut raw_instruction: String) -> String {
        let mut optimized_instruction = String::new();
        let counterclockwise_symbol = "'";
        let doubletime_symbol = "2";
        loop {
            let mut i = 0;
            let mut is_optimized = true;
            while i < raw_instruction.len() {
                if raw_instruction.len() >= i + 8
                    && (&raw_instruction[i + 2..i + 3] == &raw_instruction[i..i + 1]
                        && &raw_instruction[i + 4..i + 5] == &raw_instruction[i..i + 1]
                        && &raw_instruction[i + 6..i + 7] == &raw_instruction[i..i + 1])
                    && (&raw_instruction[i + 1..i + 2] == counterclockwise_symbol
                        && &raw_instruction[i + 3..i + 4] == counterclockwise_symbol
                        && &raw_instruction[i + 5..i + 6] == counterclockwise_symbol
                        && &raw_instruction[i + 7..i + 8] == counterclockwise_symbol)
                {
                    is_optimized = false;

                    i += 8;
                } else if raw_instruction.len() >= i + 6
                    && (&raw_instruction[i + 2..i + 3] == &raw_instruction[i..i + 1]
                        && &raw_instruction[i + 4..i + 5] == &raw_instruction[i..i + 1])
                    && (&raw_instruction[i + 1..i + 2] == counterclockwise_symbol
                        && &raw_instruction[i + 3..i + 4] == counterclockwise_symbol
                        && &raw_instruction[i + 5..i + 6] == counterclockwise_symbol)
                {
                    is_optimized = false;
                    optimized_instruction += &raw_instruction[i..i + 1];

                    i += 6;
                } else if raw_instruction.len() >= i + 4
                    && (&raw_instruction[i + 2..i + 3] == &raw_instruction[i..i + 1]
                        && (&raw_instruction[i + 1..i + 2] == counterclockwise_symbol
                            && &raw_instruction[i + 3..i + 4] == counterclockwise_symbol))
                {
                    is_optimized = false;
                    optimized_instruction =
                        optimized_instruction + &raw_instruction[i..i + 1] + doubletime_symbol;

                    i += 4;
                } else if raw_instruction.len() >= i + 4
                    && (&raw_instruction[i + 2..i + 3] == &raw_instruction[i..i + 1]
                        && (&raw_instruction[i + 1..i + 2] == doubletime_symbol
                            && &raw_instruction[i + 3..i + 4] == doubletime_symbol))
                {
                    is_optimized = false;

                    i += 4;
                } else if raw_instruction.len() == i + 4
                    && (&raw_instruction[i..i + 1] == &raw_instruction[i + 1..i + 2]
                        && &raw_instruction[i + 1..i + 2] == &raw_instruction[i + 2..i + 3]
                        && &raw_instruction[i + 2..i + 3] == &raw_instruction[i + 3..i + 4])
                {
                    is_optimized = false;
                    i += 4;
                } else if raw_instruction.len() > i + 4
                    && (&raw_instruction[i..i + 1] == &raw_instruction[i + 1..i + 2]
                        && &raw_instruction[i + 1..i + 2] == &raw_instruction[i + 2..i + 3]
                        && &raw_instruction[i + 2..i + 3] == &raw_instruction[i + 3..i + 4])
                    && (&raw_instruction[i + 4..i + 5] != doubletime_symbol
                        && &raw_instruction[i + 4..i + 5] != counterclockwise_symbol)
                {
                    is_optimized = false;
                    i += 4;
                } else if raw_instruction.len() >= i + 4
                    && ((&raw_instruction[i + 2..i + 3] == &raw_instruction[i..i + 1]
                        && &raw_instruction[i + 1..i + 2] == doubletime_symbol
                        && &raw_instruction[i + 3..i + 4] == counterclockwise_symbol)
                        || (&raw_instruction[i + 2..i + 3] == &raw_instruction[i..i + 1]
                            && &raw_instruction[i + 1..i + 2] == counterclockwise_symbol
                            && &raw_instruction[i + 3..i + 4] == doubletime_symbol))
                {
                    is_optimized = false;
                    optimized_instruction = optimized_instruction + &raw_instruction[i..i + 1];

                    i += 4;
                } else if raw_instruction.len() == i + 3
                    && (&raw_instruction[i..i + 1] == &raw_instruction[i + 1..i + 2]
                        && &raw_instruction[i + 1..i + 2] == &raw_instruction[i + 2..i + 3])
                {
                    is_optimized = false;
                    optimized_instruction = optimized_instruction
                        + &raw_instruction[i..i + 1]
                        + counterclockwise_symbol;

                    i += 3;
                } else if raw_instruction.len() > i + 3
                    && (&raw_instruction[i..i + 1] == &raw_instruction[i + 1..i + 2]
                        && &raw_instruction[i + 1..i + 2] == &raw_instruction[i + 2..i + 3])
                    && &raw_instruction[i + 3..i + 4] != counterclockwise_symbol
                    && &raw_instruction[i + 3..i + 4] != doubletime_symbol
                {
                    is_optimized = false;
                    optimized_instruction = optimized_instruction
                        + &raw_instruction[i..i + 1]
                        + counterclockwise_symbol;

                    i += 3;
                } else if raw_instruction.len() >= i + 3
                    && (&raw_instruction[i..i + 1] == &raw_instruction[i + 1..i + 2]
                        && &raw_instruction[i + 2..i + 3] == doubletime_symbol)
                {
                    is_optimized = false;
                    optimized_instruction = optimized_instruction
                        + &raw_instruction[i..i + 1]
                        + counterclockwise_symbol;

                    i += 3;
                } else if raw_instruction.len() >= i + 3
                    && (&raw_instruction[i..i + 1] == &raw_instruction[i + 1..i + 2]
                        && &raw_instruction[i + 2..i + 3] == counterclockwise_symbol)
                {
                    is_optimized = false;

                    i += 3;
                } else if raw_instruction.len() == i + 3
                    && (&raw_instruction[i..i + 1] == &raw_instruction[i + 2..i + 3]
                        && &raw_instruction[i + 1..i + 2] == doubletime_symbol)
                {
                    is_optimized = false;
                    optimized_instruction = optimized_instruction
                        + &raw_instruction[i..i + 1]
                        + counterclockwise_symbol;

                    i += 3;
                } else if raw_instruction.len() > i + 3
                    && (&raw_instruction[i..i + 1] == &raw_instruction[i + 2..i + 3]
                        && &raw_instruction[i + 1..i + 2] == doubletime_symbol)
                    && &raw_instruction[i + 3..i + 4] != counterclockwise_symbol
                {
                    is_optimized = false;
                    optimized_instruction = optimized_instruction
                        + &raw_instruction[i..i + 1]
                        + counterclockwise_symbol;

                    i += 3;
                } else if raw_instruction.len() == i + 3
                    && (&raw_instruction[i..i + 1] == &raw_instruction[i + 2..i + 3]
                        && &raw_instruction[i + 1..i + 2] == counterclockwise_symbol)
                {
                    is_optimized = false;

                    i += 3;
                    if raw_instruction.len() > i + 3
                        && &raw_instruction[i + 3..i + 4] == doubletime_symbol
                    {
                        optimized_instruction += &raw_instruction[i..i + 1];

                        i += 1;
                    } else {
                    }
                } else if raw_instruction.len() > i + 3
                    && (&raw_instruction[i..i + 1] == &raw_instruction[i + 2..i + 3]
                        && &raw_instruction[i + 1..i + 2] == counterclockwise_symbol)
                    && &raw_instruction[i + 3..i + 4] != doubletime_symbol
                {
                    is_optimized = false;

                    i += 3;
                } else if raw_instruction.len() >= i + 2
                    && (&raw_instruction[i..i + 1] == &raw_instruction[i + 1..i + 2])
                {
                    is_optimized = false;
                    optimized_instruction =
                        optimized_instruction + &raw_instruction[i..i + 1] + doubletime_symbol;

                    i += 2;
                } else {
                    optimized_instruction += &raw_instruction[i..i + 1];

                    i += 1;
                }
            }
            if is_optimized {
                break;
            }
            raw_instruction = optimized_instruction.clone();

            optimized_instruction.clear();
        }
        optimized_instruction
    }

    pub fn transmit_optimized_instructions() -> Vec<String> {
        OPTIMIZED_INSTRUCTIONS
            .get()
            .map_or_else(|| vec![], |lock| lock.read().unwrap().clone())
    }

    pub fn clear() {
        if let Some(lock) = OPTIMIZED_INSTRUCTIONS.get() {
            lock.write().unwrap().clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimize_instructions() {
        let raw_instruction = String::from("D'R'URU'U'FFFFFFFFFFFFFFFFFFFFFFFB'B2B2DF2F2F'F'F2F");
        //let raw_instructions = vec![String::from("FFFB'")];
        let actual_optimized_instruction = OptiCourier::optimize_instructions(raw_instruction);
        let expected_optimized_instruction = String::from("D'R'URU2F'B'DF");
        assert_eq!(actual_optimized_instruction, expected_optimized_instruction);

        let raw_instruction = String::from("F2D2D");
        //let raw_instructions = vec![String::from("FFFB'")];
        let actual_optimized_instruction = OptiCourier::optimize_instructions(raw_instruction);
        let expected_optimized_instruction = String::from("F2D'");
        assert_eq!(actual_optimized_instruction, expected_optimized_instruction);

        let raw_instruction = String::from("FRUR'U'F'UUU2RUR'URU2R'UUU");
        //let raw_instructions = vec![String::from("FFFB'")];
        let actual_optimized_instruction = OptiCourier::optimize_instructions(raw_instruction);
        let expected_optimized_instruction = String::from("FRUR'U'F'RUR'URU2R'U'");
        assert_eq!(actual_optimized_instruction, expected_optimized_instruction);
    }
}
