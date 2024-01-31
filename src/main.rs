use chemistru_modelling::standard_form::StandardForm;

fn main() {
    let a = StandardForm::new(2.0, 3);

    println!("a = {:?}", a);    
    println!("a² = {:?}", a.powf(2.0));    
}
