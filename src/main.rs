

mod utils;
use utils::*;
use crate::utils::threadpool::ThrdPool;

fn fun1 () -> i32 {
    let mut a = 3;
    for i in 0..11230 {
        a+=1;
    }
    a
}
fn fun2 () -> i32 {
    let mut a = 2.0;
    for i in 0..23214 {
        a+=1.4;
    }
    a as i32
}

fn fun3 () -> i32 {
    let mut a = 1.1;
    for i in 0..2112 {
        a+=0.4;
    }
    a as i32
}


fn main() {
    let tp = ThrdPool::<i32>::new(2);
    let v1 = tp.enqueue(fun1);
    let v2 = tp.enqueue(fun2);
    let v3 = tp.enqueue(fun3);
    println!("{:?} {:?} {:?}", v1,v2,v3);
    tp.dispose();
}