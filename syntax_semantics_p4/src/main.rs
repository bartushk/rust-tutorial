fn main() {
    Drops();
    iflet();
    traitobjects();
    closures();
    universal_call_syntax();
}

#[derive(Debug)]
struct HasDrop;

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

struct Firework {
    strength: i32,
}

impl Drop for Firework  {
    fn drop(&mut self){
        println!("BOOM times {}!!!", self.strength);
    }
}

fn Drops(){
    let x = HasDrop; 
    let smoke_bomb = Firework { strength: 9001 };
    let tnt = Firework { strength: 4 };
}

fn iflet(){
    let mut v = vec![1, 3, 5, 7, 11];    
    while let Some(x) = v.pop() {
        println!("{}", x);
    }
}

trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

fn do_something<T: Foo>(x: T) -> String {
    x.method()
}

fn do_something_else(x : &Foo) -> String {
    x.method()
}

fn traitobjects(){
    let x = 5u8; 
    let y = "Hello".to_string();

    println!("{}", do_something_else(&y));
    println!("{}", do_something(x));
    println!("{}", do_something(y));
}

#[derive(Debug)]
struct Val_u {
    value : i32,
}

fn call_with_one<F>(some_closure: F) -> i32 where F : Fn(i32) -> i32 {
    some_closure(1)
}

fn call_with_one_dyn(some_closure: &Fn(i32) -> i32) -> i32 {
    some_closure(1)
}

fn call_with_ref<F>(some_closure:F) -> i32
    where F: for<'a> Fn(&'a i32) -> i32 {
        let mut value = 0;
        some_closure(&value)
    }


fn factory() -> Box<Fn(i32) -> i32> {
    let num = 5;

    Box::new(move |x| x + num)
}


fn closures(){
    let plus_one = |x| x + 1;
    let plus_two = |x| {
        x + 2
    };
    assert_eq!(2, plus_one(1));
    assert_eq!(3, plus_two(1));

    //Closures with closures
    let num = 6;
    let plus_num = move |x: i32| x + num;

    plus_one(num);
    assert_eq!(11, plus_num(5));

    let mut num2 = 5;
    {
        let mut add_num = |x: i32| num2 +=x;
        add_num(5);
    }

    println!("First value: {}", num2);

    let mut num3 = 3;
    {
        let mut add_num = move |x: i32| num3 + x;
        add_num(5);
    }

    println!("Second value: {:?}", num3);

    println!("Closure value: {}", call_with_one(|x| x + 2));

    println!("Closure dynamic value: {}", call_with_one_dyn(&|x| x + 2));

    println!("Closure generic closure pointer with explicit lifetime {}", call_with_ref(|x| x + 2));

    // Function pointer is a closure qith no environment
    let func_pointer = traitobjects;

    let f = factory();

    let answer = f(1);
    
    println!("Factory value: {:?}", answer);

}

trait Foo2 {
    fn f(&self);
}

trait Bar2 {
    fn f(&self);
}

struct Baz;

impl Foo2 for Baz {
    fn f(&self) { println!("Baz's impl of Foo2!"); }
}

impl Bar2 for Baz {
    fn f(&self) { println!("Baz's impl of Bar2!"); }
}

fn universal_call_syntax(){
    let b = Baz; 
    <Baz as Foo2>::f(&b);
}
