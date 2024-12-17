#[derive(Debug)]
pub struct GeneticGroup{
    pub genetics:Vec<u32>,
}

impl GeneticGroup {
    //计算适应度
    fn calculat_fit(genetic: u32) -> u32 {
        genetic * genetic
    }
}

impl GeneticGroup {
    //创建成员
    pub fn crate_genetic(&mut self,mem:u32)
    {
        self.genetics.push(mem);
    }

    //计算适应度队列
    pub fn get_fit_list(&self)->Vec<u32>
    {
        let mut fit_vec = vec![];

        for genetic in &self.genetics{
            
            let now_fit = GeneticGroup::calculat_fit(*genetic);
            if let Some(last) = fit_vec.last(){
                fit_vec.push(now_fit+last);
            }
            else {
                fit_vec.push(now_fit);
            }
        }
        return fit_vec;
    }


}