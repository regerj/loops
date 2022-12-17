fn main() {
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Loops in assignments
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // Break is sorta like return here? I beleive the break is interpreted first, nullifying the loop
            // and then "counter * 2;" is placed after "let result = "
            break counter * 2;
        }
    };

    // Thus the above code evaluates down to 

    // let result = counter * 2;

    println!("The result is {result}");

    //////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Loop labels

    let mut count = 0;

    // Loop labels MUST begin with the single quote
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    //////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // While Loops

    let mut number = 3;
    
    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    //////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // For loops to iterate elements

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
