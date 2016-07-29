use std::mem;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Deref;
use std::rc::Rc;


static mut N: i32 = 5;
static NAME: &'static str = "Steve";

fn main() {
    const_static();
    aliases();
    casting();
    associated_types();
    ops_and_overloads();
    deref_coercions();
    macros();
}



fn const_static() {
    //must use unsafe block to access mutable static. 
    unsafe {
        N += 1;
        println!("N: {}", N);
    }
}


#[test]
fn test_attribute(){
    //attributes are defined by the compiler, cannot be custom
    //test is only compiled if cargo test is run
    assert_eq!(4, 4);
}

fn test_attribute_included(){
    #![test]
    // the exclamation mark makes this test apply to the function it's contained in
    assert_eq!(2, 2);
}


type eye32 = i32;
fn aliases(){
    // aliases are exactly equal to their original type and will assert so
    let one: i32 = 12;
    let won: eye32 = 12;
    assert_eq!(one, won);
}

fn casting(){
    // coercion is implicit
    let mut ref1: &str = "hello"; 
    let ref2: &str = ref1;

    // safe casting between types uses as
    let x: i32 = 5;
    let y: i64 = x as i64;

    let a = 300 as *const char;
    let b = a as u32;

    println!("b is : {}", b);                                

    unsafe {
        let a = [0u8, 1u8, 0u8, 0u8];
        let b = mem::transmute::<[u8; 4], u32>(a);
        println!("{}", b);

        let c: u32 = mem::transmute(a);
        println!("{}", c);
    }
    
}

trait Graph {
    type N;
    type E;

    fn has_edge(&self, &Self::N, &Self::N) -> bool;
    fn edges(&self, &Self::N) -> Vec<Self::E>;
}

struct Node;
struct Edge;

struct MyGraph;

impl Graph for MyGraph {
    type N = Node;
    type E = Edge;

    fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
        true
    }

    fn edges(&self, n: &Node) -> Vec<Edge> {
        Vec::new()
    }
}

fn distance<G: Graph>(graph: &G, start: &G::N, end: &G::N) -> u32 {
    32
}

fn associated_types(){
    //cannot create trait object from assicated type without specifying types
    let graph = MyGraph;
    let obj = Box::new(graph) as Box<Graph<N=Node, E=Edge>>;
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point{ x: self.x + other.x, y: self.y + other.y }
    }
}

impl Add<i32> for Point {
    type Output = f64;

    fn add(self, rhs: i32) -> f64 {
        123.9
    }

}

trait HasArea<T> {
    fn area(&self) -> T;
}

struct Square<T> {
    x: T,
    y: T,
    side: T
}

impl<T> HasArea<T> for Square<T> where T: Mul<Output=T> + Copy {
    fn area(&self) -> T {
        self.side * self.side
    }
}

fn ops_and_overloads(){
    let p1 = Point {x: 1, y: 0};
    let p2 = Point {x: 2, y: 3};
    let p4 = Point {x: 4, y: 5};

    let p3 = p1 + p2;

    let f1 = p4 + 2;

    println!("{:?}", p3);
    println!("Result of overload: {}", f1);

    let s = Square{
        x: 0.0f64,
        y: 0.0f64,
        side: 12.0f64,
    };

    println!("Area of s: {}", s.area());
}

struct DerefExample<T> {
    value: T,
}

impl<T> Deref for DerefExample<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}


fn foo(s: &str){
    
}

fn deref_coercions(){
    let x = DerefExample {value: 'a' };    
    assert_eq!('a', *x);

    let owned = "Hello".to_string();

    foo(&owned);

    let owned2 = "Hello".to_string();
    let counted = Rc::new(owned);

    foo(&counted);
}


macro_rules! foo {
    (x => $e:expr) => (println!("mode X: {}", $e));
    (y => $e:expr) => (println!("mode X: {}", $e));
}

fn macros(){
    foo!(y => 3); 
}
