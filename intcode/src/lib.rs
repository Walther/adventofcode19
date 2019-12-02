/// Implements the Intcode computer as specified by Advent of Code 2019.
/// Given an input Vec of integers, runs the computations and returns the result
/// integer.
pub fn compute(input: &Vec<usize>) -> usize {
    let mut punchcard = input.clone();
    let mut instruction_pointer = 0;

    loop {
        let opcode = punchcard[instruction_pointer];
        let increment; // Instruction pointer gets incremented by the amount of parameters used for a given instruction
        match opcode {
            // Addition instruction
            1 => {
                let addr1 = punchcard[instruction_pointer + 1];
                let addr2 = punchcard[instruction_pointer + 2];
                let target_addr = punchcard[instruction_pointer + 3];
                increment = 4;

                let result = punchcard[addr1] + punchcard[addr2];
                punchcard[target_addr] = result;
            }
            // Multiplication instruction
            2 => {
                let addr1 = punchcard[instruction_pointer + 1];
                let addr2 = punchcard[instruction_pointer + 2];
                let target_addr = punchcard[instruction_pointer + 3];
                increment = 4;

                let result = punchcard[addr1] * punchcard[addr2];
                punchcard[target_addr] = result;
            }
            // End of computation instruction
            99 => {
                break;
            }
            // Unknown instruction
            _ => {
                panic!("Unsupported opcode: {}", opcode);
            }
        }
        instruction_pointer += increment;
    }
    punchcard[0]
}

#[cfg(test)]
mod tests {
    #[test]
    fn day2_test1() {
        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let result = crate::compute(&input);
        assert_eq!(result, 3500);
    }

    #[test]
    fn day2_test2() {
        let input = vec![1, 0, 0, 0, 99];
        let result = crate::compute(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn day2_test3() {
        let input = vec![2, 3, 0, 3, 99];
        let result = crate::compute(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn day2_test4() {
        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let result = crate::compute(&input);
        assert_eq!(result, 30);
    }
}
