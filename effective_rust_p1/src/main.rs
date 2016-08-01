use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::sync::mpsc;
use std::num::ParseIntError;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;


fn main() {
    iterators();
    concurrency();
    concurrency2();
    error_handling();
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

fn concurrency() {
    let x = 1;

    let handle = thread::spawn( || {
        "Hello from a thread: {}"
    });

    thread::spawn( move || {
        println!("The value of x is: {}", x);
    });

    println!("Thread result: {}", handle.join().unwrap());

    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    thread::spawn(move || {
        let mut data = data.lock().unwrap();
        data[0] += 1;
    });

    thread::sleep(Duration::from_millis(50));
}

fn concurrency2() {
    let data = Arc::new(Mutex::new(0));

    // 'tx' is the "transmitter" or "sender"
    // 'rx' is the "receiver"
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let tx =  tx.clone();

        thread::spawn(move || {
            let answer = i * i;

            tx.send(answer).unwrap();
        });
    }

    for _ in 0..10 {
        println!("{}", rx.recv().unwrap());
    }

    let handle = thread::spawn( move || {
        panic!("AHHH!");
    });

    let result = handle.join();

    assert!(result.is_err());
}


fn extension(file_name: &str) -> Option<&str> {
    find(file_name, '.').map(|i| &file_name[i+1..])
}

fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}


fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
    match number_str.parse::<i32>() {
        Ok(n) => Ok(2 * n),
        Err(err) => Err(err),
    }
}

fn double_arg(mut argv: env::Args) -> Result<i32, String> {
    argv.nth(1)
        .ok_or("Please give at least one argument".to_owned())
        .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
        .map(|n| 2 * n)
}


fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    File::open(file_path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_| contents)
        })
        .and_then(|contents| {
            contents.trim().parse::<i32>()
                .map_err(|err| err.to_string())
        })
        .map(|n| 2 * n)
}

fn file_double2<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let mut file = match File::open(file_path){
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };
    
    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        return Err(err.to_string());
    }

    let n: i32 = match contents.trim().parse() {
        Ok(n) => n,
        Err(err) => return Err(err.to_string()),
    };
    Ok(2 * n)

}


fn error_handling() {
    let file_name = "foobar.rs";
    match find(file_name, ',') {
        None => println!("No file extension found."),
        Some(i) => println!("File extension: {}", &file_name[i +1..]),
    }
     
    assert_eq!(extension("foobar.csv").unwrap_or("rs"), "csv");
    assert_eq!(extension("foobar").unwrap_or("rs"), "rs");
     
    match double_number("10") {
        Ok(t) => assert_eq!(t, 20),
        Err(err) => println!("Error: {:?}", err),
    }

    let mut argv = env::args();
    match double_arg(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }

    match file_double("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err)
    }

    match file_double2("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err)
    }

}
