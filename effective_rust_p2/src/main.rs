extern crate libc;

use std::boxed::Box;
use std::cell::Cell;
use std::cell::RefCell;
use libc::{ c_int, size_t };
use std::collections::HashMap;
use std::borrow::Borrow;
use std::fmt::Display;


fn main() {
    choosing_guarantees();
    ffi();
    borrow_and_asref();
}


fn choosing_guarantees() {
    let x = Box::new(1);
    let z = x.as_ref();
    let g = x.as_ref();
    println!("Box z: {:?}", z);
    println!("Box g: {:?}", g);

    // Ownership is nolonger with x cannot do this:
    // println!("Box x: {:?}", x); 
    
    
    // restricted to copy types
    let c = Cell::new(1);
    let c2 = &c;
    let c3 = &c;

    c.set(2);
    c2.set(c.get() + 3);
    c3.set(c2.get() + 4);
    println!("{}", c.get());

    let x = RefCell::new(vec![1, 2, 3, 4]); 
    {
        println!("{:?}", *x.borrow())
    }

    {
        let mut my_ref = x.borrow_mut();
        my_ref.push(1);
    }

}

#[link(name = "snappy")]
extern {
    fn snappy_compress(input : *const u8,
                       input_length: size_t,
                       compressed: *mut u8,
                       compressed_length: *mut size_t) -> c_int;

    fn snappy_uncompress(compressed: *const u8,
                         compressed_length: size_t,
                         uncompressed: *mut u8,
                         uncompressed_length: *mut size_t) -> c_int;

    fn snappy_max_compressed_length(source_length: size_t) -> size_t;

    fn snappy_uncompressed_length(compressed: *const u8,
                                  compressed_length: size_t,
                                  result: *mut size_t) -> c_int;

    fn snappy_validate_compressed_buffer(compressed: *const u8,
                                         compressed_length: size_t) -> c_int;
                        
}

pub fn validate_compressed_buffer(src: &[u8]) -> bool {
    unsafe {
        snappy_validate_compressed_buffer(src.as_ptr(), src.len() as size_t) == 0
    }
}

pub fn compress(src: &[u8] ) -> Vec<u8> {
    unsafe {
        let srclen = src.len() as size_t;
        let psrc = src.as_ptr();

        let mut dstlen = snappy_max_compressed_length(srclen);
        let mut dst = Vec::with_capacity(dstlen as usize);
        let pdst = dst.as_mut_ptr();

        snappy_compress(psrc, srclen, pdst, &mut dstlen);
        dst.set_len(dstlen as usize);
        dst
    }
}

#[link(name = "readline")]
extern {
    static rl_readline_version: libc::c_int;
}

extern "C" {
    pub fn foo(arg: *mut libc::c_void);
    pub fn bar(arg: *mut libc::c_void);
}

pub enum Foo {}
pub enum Bar {}

extern "C" {
    pub fn foo2(arg: *mut Foo);
    pub fn bar2(arg: *mut Bar);
}

pub fn uncompress(src: &[u8]) -> Option<Vec<u8>> {
    unsafe {
        let srclen = src.len() as size_t;
        let psrc = src.as_ptr();

        let mut dstlen: size_t = 0;
        snappy_uncompressed_length(psrc, srclen, &mut dstlen);

        let mut dst = Vec::with_capacity(dstlen as usize);
        let pdst = dst.as_mut_ptr();

        if snappy_uncompress(psrc, srclen, pdst, &mut dstlen) == 0{
            dst.set_len(dstlen as usize);
            Some(dst)
        } else {
            None
        }

    }
}

fn ffi() {
    let x = unsafe { snappy_max_compressed_length(100) }; 
    println!("max compressed length of a 100 byte buffer: {}", x);
    println!("You have readline version {} installed.", rl_readline_version as i32);
}

fn foos<T: Borrow<i32> + Display>(a: T) {
    println!("a is borrowed: {}", a);
}

fn foosb<T: AsRef<str>>(s: T){

}

fn borrow_and_asref() {
    let mut map = HashMap::new(); 
    map.insert("Foo".to_string(), 42);

    let mut i = 5;
    assert_eq!(map.get("Foo"), Some(&42));
    foos(&i);
    foos(&mut i);
}
