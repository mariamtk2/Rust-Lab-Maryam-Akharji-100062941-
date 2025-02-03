//fn main() {
    
    //exercise 1
   // let x = 5;
    //println!("x has the value {x}");


    // exercise 2
    //let x=3;

    //if x == 10 {
      //  println!("x is ten!");
    //} else {
      //  println!("x is not ten!");
    //}

    // exercise 3
     //let x: i32 = 23;

     //println!("Number {x}");

    // exercise 4

   // let mut x = 3;
    //println!("Number {x}");

    //x = 5; 
    //println!("Number {x}");

    // exercise 5

    //let number = "T-H-R-E-E"; // Don't change this line
    //println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    //let number = 3;
    //println!("Number plus two is: {}", number + 2);

// exercise 6
/*const NUMBER: u64 = 3;

fn main() {
    println!("Number: {NUMBER}");
}

} */

// exercise 2

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
   
}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(vec0.clone());

    
        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
