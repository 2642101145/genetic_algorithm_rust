mod genetic_algorithm;
mod basic_struct;

fn main() {
    //创建父代基因
    let mut genetics = genetic_algorithm::creat_genetics(6);

    
    let mut loop_times = 5000;
    println!("初代Genetics: {:?}", genetics);
    while loop_times!=0 {
        //选择基因
        genetics = genetic_algorithm::chose(&genetics);

        //杂交
        let mut prob = 700;
        genetics = genetic_algorithm::mutation(genetics, prob);

        //变异
        prob = 90;
        genetics = genetic_algorithm::hybrid(genetics, prob);
        
        loop_times-=1;
    }
    println!("末代Genetics: {:?}", genetics);
}

