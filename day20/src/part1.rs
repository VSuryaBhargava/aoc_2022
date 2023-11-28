use utils::problem::SolvePart1;

use crate::parser::{ParsedOutput, Problem};

impl SolvePart1 for Problem {
    type ParsedType = ParsedOutput;

    fn solve_part_one(&mut self, (mut parsed_data, index_of_0): ParsedOutput) -> String {
        let number_of_mixes = self.number_of_mixes;
        let multiplication_factor = self.multiplication_factor;

        let arr_size = parsed_data.len();
        let max_index = (arr_size - 1) as i64;
        let mut movement_tracking_array: Vec<i64> = vec![0; arr_size];

        for iter_count in 0..(arr_size * number_of_mixes as usize) {
            let i = iter_count % arr_size;
            let mut index = i as i64 + movement_tracking_array[i];
            let mut movement =
                parsed_data[index as usize].data * (multiplication_factor % max_index);
            let mut new_index = index as i64 + movement;

            while new_index < 0 {
                new_index += max_index
            }
            while new_index > max_index {
                new_index -= max_index
            }

            movement = new_index - index;

            if movement == 0 {
                continue;
            }

            let per_itr_movement = movement / movement.abs();

            while index != new_index {
                parsed_data.swap(index as usize, (index + per_itr_movement) as usize);

                movement_tracking_array[parsed_data[index as usize].index as usize] -=
                    per_itr_movement;
                movement_tracking_array
                    [parsed_data[(index + per_itr_movement) as usize].index as usize] +=
                    per_itr_movement;

                index += per_itr_movement;
            }
        }

        let final_index_of_0 = index_of_0 + movement_tracking_array[index_of_0 as usize];

        let i1000 =
            parsed_data[(final_index_of_0 + 1000) as usize % arr_size].data * multiplication_factor;
        let i2000 =
            parsed_data[(final_index_of_0 + 2000) as usize % arr_size].data * multiplication_factor;
        let i3000 =
            parsed_data[(final_index_of_0 + 3000) as usize % arr_size].data * multiplication_factor;

        format!("{}", i1000 + i2000 + i3000)
    }
}
