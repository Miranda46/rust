fn main() {
    let number = 3;
    let condition = false;

    if number < 5 {
        println!("Yeah");
    }
    else {
        println!("Nah");
    }

    let knowledge = if condition {"Acquired"} else {"not yet"};

    println!("{}", knowledge);

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 5;
        }
    };

    println!("result {}, counter {}",result, counter);


    let mut count = 0;
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
    // loops can be named to be broken when looped
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // giving up the name, it must contain ' at the start
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");


    let list = [1,2,3,4,5,6];

    for item in list {
        println!("The value is {item}", );
    }

    for number in (0..count).rev(){
        println!("{number}!")
    }
    println!("lessgo")
}
