/*
Practicing primitives, scalars types and compound
https://doc.rust-lang.org/rust-by-example/primitives.html
*/
fn main() {

    let _entero: i32 = 12;
    let _flotante: f32 = 1.0;
    let _boleano: bool = true;
    let _caracter: &str = "hector";
    let mut caracter_mut: &str = "is not mut";
    println!("Before: {}\n",caracter_mut);
    caracter_mut = "is mut";
    
    println!("After: {}, and this other is just a name: {}",caracter_mut,_caracter);
    
    //A list
    let mut the_array: [i64;2] = [123,1234];
    println!("showing the array: {:?}",the_array);
    
    the_array = [1,2];
    println!("The list result");
    for i in the_array { println!("Número: {}",i); }
    
    let my_tuple = ("123",123,false,-123);
    println!("The tuple result:\n{:?}",my_tuple);
}