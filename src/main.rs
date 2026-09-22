fn main() {
//compound Data types (Arrays, tuples, slices, strings slicestrings)
// Arrays
let _numbers:[i32; 5] = [1,2,3,4,5];
let _fruits : [&str; 3] = ["Apple","Banana","Orange"];

// println!("First fruit is {:?}", fruits[0]);
// println!("Second fruit is {:?}", fruits[1]);
// println!("Third fruit is {:?}", fruits[2]);

//tuplles
let _human:(&str, i32, bool) = ("Alice", 30, false);
let _my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5,]);
// println!("Human tuple: {:?}", human);
// println!("mixed_tuple: {:?}", my_mix_tuple);

//slices
let number_slices: &[i32;4] = &[1,2,3,4];
println!("Number slices : {:?}", number_slices);

let animal_slices : &[&str;3] = &["Lion", "Sheep", "Cat"];
println!("animal slices : {:?}", animal_slices);

//STRING VRS STRING SLICES
let mut stone_cold: String = String::from("Hell");
stone_cold.push_str("Yeah");
println!("Stone cold says : {}", stone_cold);

// &str

let string : String = String::from("Hello, world");
let slice : &str = &string;



}
