use std::sync::{Arc, Mutex};
use std::thread;

use nonparallel::nonparallel;
use lazy_static::lazy_static;

lazy_static! { static ref MUT_A: Mutex<()> = Mutex::new(()); }
lazy_static! { static ref MUT_B: Mutex<()> = Mutex::new(()); }

const COUNT: usize = 1_000;

#[nonparallel(MUT_A)]
fn append1(vec: Arc<Mutex<Vec<u32>>>) {
    for _ in 0..COUNT {
        vec.lock().unwrap().push(1);
    }
}

#[nonparallel(MUT_A)]
fn append2(vec: Arc<Mutex<Vec<u32>>>) {
    for _ in 0..COUNT {
        vec.lock().unwrap().push(2);
    }
}

#[nonparallel(MUT_A)]
fn append3(vec: Arc<Mutex<Vec<u32>>>) {
    for _ in 0..COUNT {
        vec.lock().unwrap().push(3);
    }
}

#[test]
fn it_works() {
    let vecarc = Arc::new(Mutex::new(Vec::new()));

    let v1 = vecarc.clone();
    let t1 = thread::spawn(move || append1(v1));
    let v2 = vecarc.clone();
    let t2 = thread::spawn(move || append2(v2));
    let v3 = vecarc.clone();
    let t3 = thread::spawn(move || append3(v3));
    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();

    // Get inner vec
    let vec: Vec<_> = Arc::try_unwrap(vecarc).unwrap().into_inner().unwrap();

    // Validate vec size
    assert_eq!(vec.len(), COUNT * 3, "Vector does not contain 3*COUNT items");

    // Split vec in three. Every slice should only contain either 1, 2 or 3.
    // (depending on which function was faster).
    let mut vec1 = vec;
    let mut vec2 = vec1.split_off(COUNT);
    let vec3 = vec2.split_off(COUNT);
    assert!(vec1.iter().all(|&val| val == vec1[0]), "vec1 is {:?}", vec1);
    assert!(vec2.iter().all(|&val| val == vec2[0]), "vec2 is {:?}", vec2);
    assert!(vec3.iter().all(|&val| val == vec3[0]), "vec3 is {:?}", vec3);
}
