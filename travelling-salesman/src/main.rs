use ga_model::city::city;
use ga_model::simulation::simulation;
fn main() {
    let arr_cities = city::generat_cities(30,5.0,8.0);
    println!("{:#?}",arr_cities);
    let mut simul = simulation::Simulation::new(50,arr_cities,0.6,0.5,10);
    simul.run();

}
