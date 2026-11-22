fn main() {
    println!("Hello, world!");
    //another_function(5);
    // foo()
    //loop_label();
    while_cnt();
}

fn another_function(x: i32) {
    println!("Parameter x has value {x}");
}

fn foo() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {y}");
}

fn loop_label() {
    let mut count = 0;
    'counting_loop: loop {
        println!("count  = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // current unlabeled loop
            }
            if count == 2 {
                break 'counting_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count at {count}");
}

fn while_cnt() {
    let mut index = 1;
    while index < 5 {
        println!("INdex is {index}");
        index += 1;
    }
}
