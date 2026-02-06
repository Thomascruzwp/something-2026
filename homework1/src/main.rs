const FREEZING_FAHRENHEIT: f64 = 32.0;

//Assignment 1 
    fn fahrenheit_to_celsius(f: f64) -> f64{
     (f - FREEZING_FAHRENHEIT) * 5.0 / 9.0
 }
 fn celsius_to_fahrenheit(c: f64) -> f64{
    c * 9.0 / 5.0 + FREEZING_FAHRENHEIT
 }

//Assignment 2 
fn is_even(n: i32) -> bool{
    n % 2 == 0
}




//Assignment 3 

fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret{
        0
    }else if guess > secret{
        1


    } else{
        -1
    }
}




fn main(){
println!("Assignment 1 Run results");


    let mut temp_f: f64 = FREEZING_FAHRENHEIT;


    let temp_c = fahrenheit_to_celsius(temp_f);
    let back_to_f = celsius_to_fahrenheit(temp_c);
    println!("{temp_f}°F = {temp_c:.2}°C = {back_to_f:.2}°F");


    for _ in 0..5{
        temp_f += 1.0;
        let c = fahrenheit_to_celsius(temp_f);
        let f = celsius_to_fahrenheit(c);
        println!("{temp_f}°F = {c:.2}°C = {f:.2}°F");
    }




println!("\nAssignment 2 Run results");


let numbers = [5, 12, 7, 18, 25, 30, 1, 14, 9, 20];
println!("Number analysis:");


for &num in &numbers{
    if num % 15 == 0 {
        println!("{}: FizzBuzz", num);
    }else if num % 3 == 0 {
        println!("{}: Fizz", num);
    }else if num % 5 == 0{
        println!("{}: Buzz", num);
    }else if is_even(num){
        println!("{}: Even", num);
    }else{
        println!("{}: Odd", num);
    }
}


let mut index = 0;
let mut sum = 0;
while index < numbers.len(){
    sum += numbers[index];
    index += 1;
}
println!("Sum of all numbers: {}", sum);


let mut largest = numbers[0];
let mut i = 0;
loop{
    if numbers[i] > largest {
        largest = numbers[i];
    }
    i += 1;
    if i >= numbers.len(){
        break;
    }
}
println!("Largest number in the array: {}", largest);




println!("\nAssignment 3 Run results");


let mut secret: i32 = 42;
let guesses = [10, 50, 40, 45, 42];


let mut attempts = 0;
let mut idx = 0;


loop{
    let guess = guesses[idx];
    attempts += 1;


    let result = check_guess(guess, secret);


    if result == 0 {
        println!("Guess {} is correct!", guess);
        break;
    }else if result == 1 {
        println!("Guess {} is too high.", guess);
    }else{
        println!("Guess {} is too low.", guess);
    }
    idx += 1;
}


println!("It took {} guesses to find the secret number.", attempts);


}
