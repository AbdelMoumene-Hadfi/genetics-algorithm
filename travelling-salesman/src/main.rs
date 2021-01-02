use ga_model::city::city;
use ga_model::simulation::simulation;
use ga_model::helper::helper;

fn main() {
    let arr_cities = city::generat_cities(10,5.0,8.0);
    helper::dot_file(&arr_cities);
    let mut simul = simulation::Simulation::new(200,arr_cities,0.6,0.5,60);
    simul.run();

}
