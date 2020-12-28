use super::city::City;
use rand::Rng;
pub const MIN_POSITIVE: f64 = 2.2250738585072014e-308f64;

#[derive(Debug)]
pub struct Individual {
    pub dna : Vec<usize>,
    pub fitness : f64,
}

impl Individual {
    pub fn new(dna:Vec<usize>,cities:&[City]) -> Self {
        let fitness = fitness(&dna,&cities);
        Individual {
            dna,
            fitness
        }
    }
    pub fn crossover(&self,other:&Individual,cities:&[City]) -> (Self,Self) {
         let n = self.dna.len();
         let mut rng = rand::thread_rng();
         let start = rng.gen_range(0,n-1);
         let end = rng.gen_range(start+1,n-1);
         let first_child_dna = crossover_dna(&self.dna,&other.dna,start,end);
         let second_child_dna = crossover_dna(&other.dna,&self.dna,start,end);
         (Individual::new(first_child_dna,cities),Individual::new(second_child_dna,cities))
    }
    pub fn mutation(&mut self,cities:&[City])  {
        let i = rand::thread_rng().gen_range(0,self.dna.len()-2);
        self.dna.swap(i,i+1);
        self.fitness=fitness(&self.dna,&cities);
    }

}

fn crossover_dna(first_parent:&[usize],second_parent:&[usize],start:usize,end:usize) -> Vec<usize> {
    let second_slice=&second_parent[start..=end];
    let mut child:Vec<usize>=Vec::new();
    for i in 0..first_parent.len() {
        if !second_slice.contains(&first_parent[i]) {
            child.push(first_parent[i]);
        }
    }
    let end_slice = &child.split_off(start);
    child.extend_from_slice(second_slice);
    child.extend_from_slice(end_slice);
    child
}

fn fitness(dna:&[usize],cities:&[City]) -> f64 {
    let f = dna.windows(2).fold(MIN_POSITIVE,|acc,x| acc+cities[x[0]].distance(&cities[x[1]]));
    f/1.0
}
