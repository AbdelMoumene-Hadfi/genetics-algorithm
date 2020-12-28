use rand::Rng;

#[derive(Debug)]
pub struct City {
    id: usize,
    x : f64,
    y : f64,
}

impl City {
    pub fn new(id: usize,x : f64,y : f64) -> Self {
        City {
            id,
            x,
            y
        }
    }
    pub fn distance(&self,other : &City) -> f64 {
        (self.x-other.x).powi(2)+(self.y-other.y).powi(2)
    }
}

pub fn generat_cities(n:usize,mx:f64,my:f64) -> Vec<City> {
     let mut rng = rand::thread_rng();
     let mut cities:Vec<City> = Vec::new();
     for i in 0..n {
         cities.push(City::new(i,rng.gen_range(0.0,mx),rng.gen_range(0.0,my)));
     }
     cities
}
