
fn main() {
    iterators();
}

/// Used for the iterator section of efective rust.
///
/// I have seperated the different sections of the book into their own functions, where 
/// I test the functionality learned in that particular section. This is the iterator section.
///
/// # Example
///
/// ```
/// iterators();
/// ```
fn iterators() {
    let mut range = 0..10;

    loop{
        match range.next(){
            Some(x) => {
                println!("{}", x);
            },
            None => { break }
        }
    }

    let greater_than_forty_two = (0..22).find(|x| *x > 42);
    let sum = (1..4).fold(0, |sum, x| sum + x);
    let map = (10..20).map(|x| x * 2 );
    let take = (1..).take(5).map(|x| x + 2);
    let filter = (1..10).filter( |&x| x < 5);

    for i in take {
        println!("Take: {}", i);
    }

    for i in filter {
        println!("Filter: {}", i);
    }

    println!("Greater than: {:?}", greater_than_forty_two);
    println!("Greater than: {:?}", map);
    println!("Sum: {}", sum);
    match greater_than_forty_two {
        Some(_) => println!("Found a match!"),
        None => println!("No match found :("),
    }
}
