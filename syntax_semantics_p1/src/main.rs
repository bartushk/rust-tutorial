
fn main() {
    // this is a line comment
    let f : fn(i32) -> i32;
    f = add_one;
    vb_basics();
    vb_shadowing();
    print_number(4);
    let mut x = add_one(3);
    println!("Add one result: {}", x);
    x = early_return(-1);
    println!("Early return result: {}", x);
    x = f(11);
    println!("Function pointer add_one {}.", x);
    primitive_types();
    if_statements();
    loops();
    vectors();
}


fn vb_basics(){
    //pattern matching with bindings
    let (x, y) = (1, 2); 
    let mut z : i32 = 5;
    println!("x: {}, y: {}, z: {}", x, y, z);
    z = 32;
    println!("new z: {}", z);

}

fn vb_shadowing(){
    let x: i32 = 17;
    {
        let y: i32 = 3;
        println!("The vlaue of x is {} and the value of y is {}", x, y);
        let x: i32 = 4;
        println!("X is shadowed: {}", x);
    }
    
    // x shadowed in ain if statement.
    if x < 18 {
        let x: i32 = 7;
        println!("X is shadowed: {}", x);
    }
    // y is out of scope
    //println!("The vlaue of x is {} and the value of y is {}", x, y);
    println!("X original {}", x);

}

fn print_number(x: i32){
    println!("x is: {}", x);
}

/// Function that adds one to an i32
///
/// # Examples
///
/// ...
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #     x + 1
/// # }
/// ...
fn add_one(x: i32) -> i32 {
    x + 1
}

fn early_return(x: i32) -> i32{
    if( x < 0 ){
        return x * -1;
    }
    x
}

fn diverges() -> ! {
    panic!("PANIC!");
}

fn primitive_types() {
    //bool
    let b_x = true;
    let b_y: bool = false;
    
    let x: char = 'x';
    let t_h: char = 'h';
    println!("Chars: x: {}. t_h: {}", x, t_h);


    //arrays
    let a: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let a2: [&str; 3] = ["one", "two", "three"];
    println!("Array: {:?}", a);

    //Slice array
    let slice = &a[3..6];
    println!("Slice: {:?}", slice);


    //Tuples
    let t1: (i32, &str) = (1, "hello");
    let mut t2 = (2, "goodbye");
    println!("T2: {:?}", t2);
    t2 = t1;
    println!("T2: {:?}", t2);
    let (tt1, tt2) = t2;
}


fn if_statements(){
    let x = 6;
    if x == 5 {
        println!("x is five!");
    } else {
        println!("x is not five!");
    }

    let y = if x == 6 {10} else {15};
}

fn loops(){
    //loop - it's infinite
    let mut i: i32 = 2;
    loop {
        i = i + 1;
        if i > 5 { break; }
    }

    //while - takes a condition
    let mut done = false;
    let mut x = 5;
    while !done {
        x += x - 1;

        if x % 5 == 0 {
            done = true;
            println!("Done x={}", x);
        }
    }

    for x in 0..10 {
        println!("{}", x);
    }

    for (i, j) in (100..110).enumerate(){
        println!("i = {} and j = {}", i , j);
    }

    let a = ["asdF", "fdsfd", "fdfsf"];
    for (i, val) in a.iter().enumerate() {
        println!("i = {} val = {}", i, val);
    }

    for x in 0..10 {
        if x % 2 == 0 { continue; }

        println!("X odd: {}", x);
    }

    'outer: for x in 0..4 {
        'inner: for y in 0..4 {
            if x % 2 == 0 { continue 'outer; }
            if y % 2 == 0 { continue 'inner; }
            println!("x: {}, y: {}", x, y);

        }
    }
}


fn vectors(){
    // vectors have changing size and are typed    
    let mut v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![1; 10];
    println!("v2: {:?}", v1);
    println!("v2 at 2: {}", v2[2]);
    // must index with usize type
    let j: usize = 2;
    let k: i32 = 2;
    //ok
    v1[j];
    //not ok
    //v1[k];
    
    //handling vector out of bounds panics
    match v1.get(7) {
        Some(x) => println!("Item 7 is {}", x),
        None => println!("Sorry, this vector is too short.")
    }

    for i in &v1 {
        println!("A reference reference to {}", i);
    }
    
    for i in &mut v1 {
        println!("A mutable reference to {} ", i);
    }

    // after taking ownership, you cannot iterate again
    for i in v1 {
        println!("Take ownership of {}", i);
    }

}
