use std::net::TcpStream;

fn main() {
    method_syntax();
    strings();
    generics();
    traits();
}

struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl CircleBuilder {
    fn new() -> CircleBuilder {
        CircleBuilder { x: 0.0, y: 0.0, radius: 1.0, }
    }

    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }

    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.y == coordinate;
        self
    }
        
    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }

    fn finalize(&self) -> Circle {
        Circle{ x: self.x, y: self.y, radius: self.radius }
    }
}


#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}


impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x, 
            y: y,
            radius: radius,
        }
    }

    fn reference(&self) {
        println!("taking self by reference!");
    }

    fn mutable_reference(&mut self) {
        println!("taking self by mutable reference!");
    }

    fn takes_ownership(self) {
        println!("taking ownership of self!");
    }

    fn grow(&self, increment: f64) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius + increment }
    }
}



fn method_syntax(){
    let mut c = Circle{ x: 0.0, y: 0.0, radius: 2.0 };    
    println!("Circle area: {}", c.area());
    c.reference();
    c.mutable_reference();
    c.takes_ownership();
    
    let c2 = Circle::new(0.0, 0.0, 1.0);
    let d = c2.grow(2.0).area();
    println!("Result of d: {}", d);

    let c3 = CircleBuilder::new()
                    .x(1.0)
                    .y(1.0)
                    .radius(2.0)
                    .finalize();

    println!("Circle from builder: {:?}", c3);
}

fn strings(){
    let s = "foo
            bar";
    let s2 = "foo\
                bar";
    let mut s3 = String::new();
    s3.push_str("asdfasdf\n");
    s3.push_str("string two");
    println!("{}", s);
    println!("{}", s2);
    test_str_func(&s3);

    //TcpStream::connect("192.168.0.1:3000");

    //let addr_string = "192.168.0.1:3000".to_string();
    //TcpStream::connect(&*addr_string);

    let hachiko = "忠犬ハチ公";

    for b in hachiko.as_bytes(){
        print!("{}, ", b);
    }

    println!("");

    for b in hachiko.chars(){
        print!("{}", b);
    }

    println!("");
    println!("Get a single char: {:?}", hachiko.chars().nth(2));
    // Character slicing uses byte indexes
    //
    println!("2nd char: {}", &hachiko[3..6]);

    let hello = "Hello ".to_string();
    let world = "world!";

    let hello_world = hello + world;

    let hello2 = "Hello ".to_string();
    let world2 = "world!".to_string();

    let hello_world2 = hello2 + &world2;
    println!("{}", hello_world);
    println!("{}", hello_world2);

}

fn test_str_func(string: &str){
    println!("{}", string);
}


#[derive(Debug)]
enum Answer<T, V>{
    good(T),
    okay(T),
    bad(V)
}

fn test_num(to_test: i32) -> Answer<i32, &'static str> {
    match to_test {
        to_test if to_test > 2 => Answer::good(to_test),
        to_test if to_test < -2 => Answer::okay(to_test),
        _ => Answer::bad("I don't like that number."),
    }
}

fn return_generic<T>(item: T) -> T {
    item
}

struct TwoItems<T, V>{
    item1: T,
    item2: V
}

impl<T, V> TwoItems<T, V>  where T: std::fmt::Display, V: std::fmt::Display {
    fn printthem(&self){
        println!("Item1: {}   Item2: {}", self.item1, self.item2);
    }
}


fn generics(){
    let a1 = test_num(5);
    let a2 = test_num(-5);
    let a3 = test_num(0);
    println!("A1: {:?}", a1);
    println!("A2: {:?}", a2);
    println!("A3: {:?}", a3);
    let my_struct = TwoItems{ item1: "asdf", item2: 4 };
    my_struct.printthem();
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct Square {
    x: f64,
    y: f64,
    side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: HasArea>(shape: T){
    println!("This shape has an area of {}", shape.area());
}


trait Fooo {
    fn is_valid(&self) -> bool;

    fn is_invalid(&self) -> bool { !self.is_valid() }
}

struct UseDefault;

impl Fooo for UseDefault {
    fn is_valid(&self) -> bool {
        println!("Called UseDefault.is_valid.");
        true
    }
}

struct OverrideDefault;

impl Fooo for OverrideDefault {
    fn is_valid(&self) -> bool {
        println!("Called OverrideDefault.is_valid");
        true
    }

    fn is_invalid(&self) -> bool {
        println!("Called OverrideDefault.is_invalid!");
        true
    }
}

struct Baz;

trait Foo {
    fn foo(&self);
}

trait FooBar : Foo {
    fn foobar(&self);
}

impl Foo for Baz {
    fn foo(&self){ println!("foo"); }
}

impl FooBar for Baz {
    fn foobar(&self) { println!("foobar"); }
}

fn traits(){
    let s = Square{x: 3.0, y:3.0, side: 9.0};
    let c = Circle{x: 3.0, y:3.0, radius: 9.0};

    let default = UseDefault{};
    let default2 = OverrideDefault{};

    default.is_valid();
    default.is_invalid();

    print_area(s);
    print_area(c);

    let b = Baz{};
    b.foo();
}
