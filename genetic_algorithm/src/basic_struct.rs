const GENETIC_LEN: u32 = 5;

#[derive(Debug)]
pub struct GeneticGroup{
     genetics:Vec<u32>//基因种群
}

impl GeneticGroup {
    //计算适应度
    fn calculat_fit(genetic: u32) -> u32 {
        genetic * genetic
    }

    fn exchange_bits(num_a: u32, num_b: u32, n: u32) -> (u32, u32) {
        let mask = (1 << n) - 1; // 创建掩码，获取从第n位开始的位
        let bits_a = num_a & mask; // 获取num_a的n位
        let bits_b = num_b & mask; // 获取num_b的n位
        
        if bits_a == bits_b
        {
            return (num_a, num_b);
        }

        // 交换位
        let new_a = (num_a & !mask) | bits_b; // 将num_a的n位替换为num_b的n位
        let new_b = (num_b & !mask) | bits_a; // 将num_b的n位替换为num_a的n位

        (new_a, new_b)
    }
}

impl GeneticGroup {
    // 新增构造函数
    pub fn new() -> Self {
        GeneticGroup {
            genetics: Vec::new()
        }
    }

    //创建成员
    pub fn crate_genetic(&mut self,mem:u32)
    {
        self.genetics.push(mem);
    }

    //获取成员
    pub fn genetics(&self) -> &Vec<u32>
    {
        &self.genetics
    }

    //设置成员
    #[allow(dead_code)]
    pub fn mut_genetics(&mut self) -> &mut Vec<u32>
    {
        &mut self.genetics
    }

    pub fn get_genetics_len(&self)->u32
    {
        self.genetics.len() as u32
    }

    //获取基因长度
    pub fn get_genetic_len()->u32
    {
        GENETIC_LEN
    }

    //计算适应度队列
    pub fn get_fit_list(&self)->Vec<u32>
    {
        let mut fit_vec = vec![];

        for genetic in &self.genetics{
            
            let mut now_fit = GeneticGroup::calculat_fit(*genetic);
            if let Some(last) = fit_vec.last(){
                now_fit+=last;
            }
            fit_vec.push(now_fit);
        }
        return fit_vec;
    }

    //按照idx获取种群的基因
    pub fn get_genetic_mem(&self, mem_idx: u32) -> Option<u32>
    {
        if let Some(&genetic_value) = self.genetics.get(mem_idx as usize) {
            Some(genetic_value)
        } else {
            None
        }
    }

    //mem_a,mem_b从第n位开始交换基因
    pub fn exchange_mem_fragments(&self, mem_a_idx: u32, mem_b_idx: u32, begin_point: u32) -> (u32, u32)
    {
        let mem_a = self.get_genetic_mem(mem_a_idx).unwrap_or(0);
        let mem_b = self.get_genetic_mem(mem_b_idx).unwrap_or(0);
        Self::exchange_bits(mem_a, mem_b, begin_point)
    }

    //mem第n位翻转
    pub fn invert_mem_bits(&self,mem_idx: u32,invert_idx:u32)->u32
    {
        let mem = self.get_genetic_mem(mem_idx).unwrap_or(0);
        let mask = 1 << invert_idx;
        mem ^ mask
    }

}