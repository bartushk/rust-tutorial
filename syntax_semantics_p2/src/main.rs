use std::sync::Arc;
use std::cell::RefCell;
use std::cell::Cell;

fn main() {
    ownership();
    borrowing();
    lifetimes();
    mutability();
    structs();
    enums();
    matches();
    patterns();
}


fn ownership(){
    let v = vec![1, 2, 3];
    let v2 = v;

    //Does not work, v no longer has ownership
    //println!("v[0] is: {}", v[0]);
    println!("v[0] is: {}", v2[0]);
    take(v2);
    //Does not work, v is now owned by 'take' function
    //println!("v[0] is: {}", v2[0]);
    
    //Copy types do not use 'references' and can be passed around
    let a = true;
    change_truth(a);
    // change_truth does not take ownership of copy type.
    println!("A value: {}", a);

}

fn take(v: Vec<i32>) -> usize {
    return v.len(); 
}

fn change_truth(x: bool) -> bool {
    !x
}

fn borrowing(){
    let v1 = vec![1, 2, 3];
    let v2 = vec![3, 4, 5];

    //Pass by reference
    let s = foo_vec(&v1, &v2);
    println!("The sum is: {}", s);

    //Mutable reference
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("X back in scope: {}", x);
}

fn sum_vec(v: &Vec<i32>) -> i32 {
    return v.iter().fold(0, |a, &b| a + b);
}

fn foo_vec(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    let s1 = sum_vec(v1);
    let s2 = sum_vec(v2);
    s1 + s2
}

struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a>{
    fn x(&self) -> &'a i32 { self.x }
}

fn lifetimes(){
    let z: i32;
    {
        let y = &5;
        let f = Foo { x: y };
        // lifetime of f is not long enough to asign to z
        //z = &f.x;
    }
}

fn frob<'a>(s: &'a str, t: &'a str) -> &'a str {
    s
}

struct FieldMutableStruct{
    field1: i32,
    field2: Cell<i32>
}

fn mutability(){
    let x = Arc::new(5);    
    let y = x.clone();

    let a = RefCell::new(42);
    let b = a.borrow_mut();
    // runtime panic:
    //let c = a.borrow_mut();
    
    //Field level mutability
    let point = FieldMutableStruct{ field1: 5, field2: Cell::new(6) };
    point.field2.set(7);
    println!("Point struct had one field mutated: {:?}", point.field2.get());
}

#[derive(Debug)]
struct Point{
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Point3d{
    x: i32,
    y: i32,
    z: i32,
}


fn structs(){
    let origin = Point{ x: 0, y: 0}; 
    println!("Point struct {:?}", origin);
    //Causes an error because structs' mutability is based off the binding
    //origin.x = 12;
          
   
    // Inference of what to copy
    let origin3d = Point3d{x:0, y:0, z:0};
    let point3d = Point3d{ z: 1, .. origin3d};
    println!("Point3d copy constructed {:?}", point3d);
}

#[derive(Debug)]
enum Message{
    Quit,
    ChangeColor(i32, i32, i32),
    Move {x: i32, y: i32},
    Write(String),
}

#[derive(Debug)]
enum BoardGameTurn {
    Move { squares: i32 },
    Pass,
}

fn enums(){
    let m1 : BoardGameTurn = BoardGameTurn::Move{ squares: 1}; 
    let mes: Message = Message::Move{ x:3, y: 4};
    println!("Enum value: {:?}", m1);
    println!("Enum value: {:?}", mes);
}

fn matches(){
    let x : i32= 5;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }

    let m1: Message = Message::Move{ x:3, y: 4};
    match m1 {
        Message::Quit => println!("Quit message."),
        Message::ChangeColor(r, g, b) => println!("Color is: {} {} {}", r, g, b),
        Message::Move{ x: x, y: y } => println!("Move to: {} {}", x, y),
        Message::Write(s) => println!("{}", s),
    };
}

enum OptionalInt{
    value(i32),
    missing,
}

fn patterns(){
    // can match multiple patterns
    let x = 1;
    match x {
        1 | 2 => println!("One or Two."),
        _ => println!("Not one or two."),
    };

    let origin = Point{x: 0, y: 0};
    match origin {
        Point{ y, .. } => println!("y is {}", y),
    }

    let mut y = 5;

    match y {
        ref r => println!("Got a reference to {}", r)
    }

    match y {
        ref mut r => println!("Got a mutable reference to {}", r)
    }

    // match a range
    match x {
        1 ... 5 => println!("one through five"),
        _ => println!("anyting else")
    }

    let g = OptionalInt::value(5);
    match g {
        OptionalInt::value(i) if i > 5 => println!("Got an int bigger than five!"),
        OptionalInt::value(..) => println!("Got an int less than five!"),
        OptionalInt::missing => println!("No such luck.")
    }
}
