use std::collections::{HashMap, HashSet}; 
use std::io::{stdin, stdout, Write};

/***
 * 
 * 
    struct Rectangle{

    }

 * 
 * 
 * 
 * ***/

struct User{

    name: String,
    ti: u64, 
    email: String,
    sign_in_count: u64,

}


fn build_user(email: String, name: String) -> User{

    // Función que retorna un Usuario con name = name e email = email

    User{

        name,
        ti:123,
        email,
        sign_in_count:1,
    }

}

fn main() {


    let width = 30;
    let height = 50;

    println!("The area of the triangle is {} square pixels", width*height/2);
    
    let user1 = User {

        name: String::from("Carmen Johana Calderón Chona"),
        email: String::from("carmenj@gmail.com"),
        ti: 12345678,
        sign_in_count:1,
    };

    let user2 = User{
        email: String::from("user2@gmail.com"),
        ..user1
    };


    println!("The value for the second user is: {0}", user2.email);

    println!("The value for the first user is: {0}", user1.ti); //Aquí el único valor que no se podría volver a llamar es el de user1.name

    
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    let x = x + 1;

    {   

        let x = x * 3;
        
        let mut y: &str = "Cadena de texto";
        let my_string: String =  String::from("Redefiniendo el tamaño de la cadena de texto");
        println!("The value of x is: {x} {y}");
        
        let mut my_int: i32 = 7;
        my_int = my_int + 4;
        println!("Valor de my_int: {my_int}");
        println!("Valor de my_int: {}", my_int-1);
        println!("Valor de my_int: {my_int}");

        let my_float: f64 = 6.4;      
        println!("{my_float}");  
        // my_float = my_float + my_int; esto no se permite
        
        let my_bool: bool = false;
        println!("Tipo booleano: {my_bool}");


        // Constantes
        const MY_CONST: &str = "Mi propiedad constante";
        println!("{MY_CONST}");

        // Control de flujo

        if my_int == 11 && my_string == "Hola" {

            println!("El valor es 11 y my_string vale Hola");

        } else if my_int == 9 || my_string == "Hola" {

            println!("El valor es 9 o el valor de my_string es Hola");


        } else {

            println!("El valor no es 11 ni 9");

        }

        // Estructuras de datos
        let mut my_list: Vec<&str> = vec!["A", "B", "C"]; // No se permiten datos de diferentes tipos
        println!("{:?}", my_list);
        my_list.push("D");
        println!("{}", my_list[3]);


        // Sets

        let mut my_set: HashSet<&str> = vec!{"E", "F", "H", "I", "J"}.into_iter().collect();
        my_set.insert("H");
        println!("{:?}", my_set);

        // map

        let mut my_map: HashMap<&str, i32> = vec![("A", 27)].into_iter().collect();
        my_map.insert("lemi_n_n", 23);
        println!("{:?}", my_map);

        // Bucles

        for value in &my_list {
            println!("{}", value);
        }


        for value in &my_set {
            println!("{}", value);
        }

        for (key, val) in my_map.iter() {
            println!("key: {key} val: {val}");
        }


        let mut my_counter: usize  = 0;
        
        
        while my_counter < my_list.len() {
            println!("{}", my_list[my_counter]);
            my_counter += 1;
        }

    }
    println!("The value of x is: {x}");
    

    // Funciones    
    my_function();

    // Structs

    let mut s = String::new();
    print!("Please enter some text: ");
    let _=stdout().flush();

    stdin().read_line(&mut s).expect("Did not enter a correct string");

    //println!("{}", s[0]);

}


fn my_function() {

    println!("Esto es una función");

}
