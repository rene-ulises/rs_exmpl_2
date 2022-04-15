use std::io::stdin;
fn main() {
    println!("This program sums two int values enter by the user");
    let val1:i32=read_int_from_usr();
    let val2:i32=read_int_from_usr();
    let result:i32=val1+val2;
    print!("The sum is: {}",result);
}

fn read_int_from_usr()->i32{
    println!("Enter an integer value: ");
    let mut in_str1 = String::new();
    stdin().read_line(&mut in_str1).expect("failed to read input.");
    let par_in: i32 = in_str1.trim().parse().expect("invalid input");
    par_in
}