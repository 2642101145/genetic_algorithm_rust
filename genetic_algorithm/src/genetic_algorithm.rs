use crate::basic_struct::GeneticGroup;
use rand::Rng;

//创建种群成员
pub fn creat_genetics(mem_num: u8) -> GeneticGroup {
    let mut member = GeneticGroup {
        genetics: vec![], // Start with an empty vector
    };
    
    let mut count = mem_num; // Create a mutable copy of mem_num
    while count != 0 {
        member.crate_genetic(rand::thread_rng().gen_range(0..=31));
        count -= 1;
    }
    return member;
}

fn get_roulette_position(fit_vec: &Vec<u32>) -> Option<usize> {
    if let Some(last) = fit_vec.last() {
        let rand_num = rand::thread_rng().gen_range(1..=*last);
        for (index, fit_mem) in fit_vec.iter().enumerate() {
            if rand_num < *fit_mem { // Dereference fit_mem
                return Some(index); // Return the position
            }
        }
    }
    None // Return None if no position is found
}

pub fn chose(genetics: GeneticGroup) -> GeneticGroup {
    let fit_vec = genetics.get_fit_list();
    // Get the length of the genetics vector
    let mut loop_times = genetics.genetics.len();
    let mut new_genetics = GeneticGroup { genetics: vec![] };
    while loop_times != 0 {
        if let Some(roulette_position) = get_roulette_position(&fit_vec) {
            if let Some(&genetic_value) = genetics.genetics.get(roulette_position) {
                new_genetics.genetics.push(genetic_value);
            }
            else {
                new_genetics.genetics.push(0);
            }
        }
        loop_times -= 1;
    }
    return new_genetics;
}