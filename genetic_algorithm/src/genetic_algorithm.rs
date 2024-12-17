use crate::basic_struct::GeneticGroup;
use rand::Rng;


//创建种群成员，mem_num必须为复数，后续交叉函数扩充处理后可为大于1的任意实数
pub fn creat_genetics(mem_num: u8) -> GeneticGroup {
    if mem_num%2 != 0 {
        panic!("Error: mem_num 必须是一个复数.");
    }
    let mut member = GeneticGroup::new();
    
    let mut count = mem_num; 
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
            if rand_num < *fit_mem { 
                return Some(index); 
            }
        }
    }
    None 
}

pub fn chose(genetics: &GeneticGroup) -> GeneticGroup {
    let fit_vec = genetics.get_fit_list();
    // Get the length of the genetics vector
    let mut loop_times = genetics.genetics().len();
    let mut new_genetics = GeneticGroup::new();
    while loop_times != 0 {
        if let Some(roulette_position) = get_roulette_position(&fit_vec) {
            if let Some(&genetic_value) = genetics.genetics().get(roulette_position) {
                new_genetics.crate_genetic(genetic_value);
            }
            else {
                new_genetics.crate_genetic(0);
            }
        }
        loop_times -= 1;
    }
    return new_genetics;
}

//杂交，概率区间[0,1000]
pub fn mutation(genetics: GeneticGroup,prob:u32)-> GeneticGroup {
    
    if prob!=0
    {
        let mut new_genetics = GeneticGroup::new();
        let mut loop_times:u32 = 0;
        while loop_times < genetics.get_genetics_len() {
            let rand_num = rand::thread_rng().gen_range(1..1000);

            let mut mem_a = genetics.get_genetic_mem(loop_times).unwrap_or(0);
            let mut mem_b = genetics.get_genetic_mem(loop_times + 1).unwrap_or(0);
            //发生杂交
            if rand_num<=prob
            {
                let begin_point = rand::thread_rng().gen_range(0..GeneticGroup::get_genetic_len());
                let result = genetics.exchange_mem_fragments(loop_times,loop_times + 1,begin_point);
                mem_a = result.0;
                mem_b = result.1;
            }

            new_genetics.crate_genetic(mem_a);
            new_genetics.crate_genetic(mem_b);

            loop_times += 2;
        }
        return new_genetics;
    }
    //概率为0不杂交
    return genetics;
}

//变异，概率区间[0,1000]
pub fn hybrid(genetics: GeneticGroup,prob:u32)->GeneticGroup{
    if prob!=0{
        let mut new_genetics = GeneticGroup::new();
        let mut loop_times:u32 = 0;
        while loop_times < genetics.get_genetics_len() {
            let rand_num = rand::thread_rng().gen_range(1..1000);
            let mut mem = genetics.get_genetic_mem(loop_times).unwrap_or(0);
            //发生变异
            if rand_num<=prob
            {
                let hybrid_point = rand::thread_rng().gen_range(0..GeneticGroup::get_genetic_len());
                mem = genetics.invert_mem_bits(loop_times, hybrid_point);
            }
            new_genetics.crate_genetic(mem);
            loop_times+=1;
        }
        return new_genetics;
    }
    


    return genetics;
}