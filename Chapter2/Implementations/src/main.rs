fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    let x = x + 1;

    {   

        let x = x * 3;
        println!("The value of x is: {x}");
    }



    println!("The value of x is: {x}");

}

