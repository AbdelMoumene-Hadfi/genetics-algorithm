use crate::city::city::City;
use crate::individual::individual::Individual;
use crate::helper::helper;
use rand::{Rng,thread_rng};
use rand::seq::SliceRandom;

pub struct Simulation {
    iteration:usize,
    cities : Vec<City>,
    population_size : usize,
    crossover_prob : f64,
    mutation_prob : f64,
    count_mutation: usize,
    count_crossover: usize,
}

impl Simulation {
    pub fn new(iteration:usize,cities:Vec<City>,crossover_prob:f64,mutation_prob:f64,population_size:usize) -> Self {
        let count_mutation:usize = 0;
        let count_crossover:usize = 0;
        Simulation {
            iteration,
            cities,
            crossover_prob,
            mutation_prob,
            population_size,
            count_mutation,
            count_crossover,
        }
    }
    fn generat_population(&mut self,population : Vec<Individual>) -> Vec<Individual> {
        let mut new_population:Vec<Individual> = Vec::new();
        let weights = cumulat_weight(&population);
        for _ in 0..(self.population_size/2) {
            let (first_parent,second_parent) = select_parents(&weights,&population);
            let (mut first_child,mut second_child) = self.generat_child(&first_parent,&second_parent);
            self.migh_mutate(&mut first_child);
            self.migh_mutate(&mut second_child);
            new_population.push(first_child);
            new_population.push(second_child);
        }
        new_population
    }
    pub fn run(&mut self) {
        let mut population = random_population(self.population_size,&self.cities);
        let mut champion = find_fittest(&population);
        for _ in 0..self.iteration {
            println!("{:?}",champion);
            population = self.generat_population(population);
            let chall = find_fittest(&population);
            if champion.fitness > chall.fitness {
                champion = chall;
            }
        }
        println!("------------------");
        println!("Result");
        println!("------------------");
        println!("Iteration : {:?}",self.iteration);
        println!("crossover probability : {:?}",self.crossover_prob);
        println!("mutation probability : {:?}",self.mutation_prob);
        println!("population size : {:?}",self.population_size);
        println!("number of cities : {:?}",self.cities.len());
        println!("Best Travel Path Founded : {:?}",champion.dna);
        println!("Fitness score : {:?}",champion.fitness);
        println!("number of crossover : {:?}",self.count_crossover);
        println!("number of mutation : {:?}",self.count_mutation);

    }
    fn generat_child(&mut self,first_parent:&Individual,second_parent:&Individual) -> (Individual,Individual){
        if thread_rng().gen_bool(self.crossover_prob) {
            self.count_crossover+=2;
            first_parent.crossover(second_parent,&self.cities)
        } else {
            (first_parent.clone(),second_parent.clone())
        }
    }
    fn migh_mutate(&mut self,child:&mut Individual) {
        if thread_rng().gen_bool(self.mutation_prob) {
            child.mutation(&self.cities);
            self.count_mutation+=1;
        }
    }


}
fn random_population(population_size:usize,cities:&[City]) -> Vec<Individual> {
    let mut populations:Vec<Individual> = Vec::new();
    let len_cities = cities.len();
    for _ in 0..population_size {
        let dna   = random_dna(len_cities);
        let indiv = Individual::new(dna,&cities);
        populations.push(indiv);
    }
    populations
}

fn random_dna(n:usize) -> Vec<usize>{
    let mut dna:Vec<usize> = (0..n).collect();
    dna.shuffle(&mut thread_rng());
    dna
}

fn select_parents<'a>(weights:&[f64],population:&'a[Individual]) -> (&'a Individual,&'a Individual) {
    let first_index = helper::select_index(weights);
    let second_index = helper::select_index(weights);
    (&population[first_index],&population[second_index])
}

fn cumulat_weight(population:&[Individual]) -> Vec<f64> {
    let mut weights:Vec<f64>= Vec::new();
    let mut sum:f64 = 0.0;
    weights.push(sum);
    for indiv in population {
        sum+=indiv.fitness;
        weights.push(sum);
    }
    weights
}

fn find_fittest(population:&[Individual]) -> Individual {
    let mut best_indiv = &population[0];
    for indiv in population {
        if indiv.fitness<best_indiv.fitness {
            best_indiv = indiv;
        }
    }
    best_indiv.clone()
}
