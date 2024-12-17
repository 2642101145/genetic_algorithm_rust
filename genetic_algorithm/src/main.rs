mod genetic_algorithm;
mod basic_struct;

fn main() {
    //创建父代基因
    let genetics = genetic_algorithm::creat_genetics(5);
    println!("New Genetics: {:?}", &genetics);
    //选择基因
    let new_genetics = genetic_algorithm::chose(genetics); // Capture the returned GeneticGroup
    println!("New Genetics: {:?}", new_genetics);
    //杂交
    
    //变异
}

