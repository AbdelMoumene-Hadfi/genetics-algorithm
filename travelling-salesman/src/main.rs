mod individual;
mod city;
fn main() {
    let arr_cities = city::generat_cities(5,5.0,8.0);
    println!("{:#?}",arr_cities);
    let indiv2 = individual::Individual::new([1,2,0,3,4].to_vec(),&arr_cities);
    println!("{:#?}",indiv2);
    let indiv3 = individual::Individual::new([3,2,0,1,4].to_vec(),&arr_cities);
    println!("{:#?}",indiv3);
    let (indiv,indiv1) = indiv2.crossover(&indiv3,&arr_cities);
    println!("{:#?}",indiv);
    println!("{:#?}",indiv1);
    let indiv5 = individual::Individual::new([3,1,0,4,2].to_vec(),&arr_cities);
    println!("{:#?}",indiv5);

}
