#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-demo/include/FiBA.h");
        
        type FiBA_SUM;

        fn create_fiba_with_sum() -> UniquePtr<FiBA_SUM>;

        fn evict(self: Pin<&mut FiBA_SUM>);
        fn insert(self: Pin<&mut FiBA_SUM>, time: &u64, value: &u64);
        fn query(&self) -> u64;

        fn oldest(&self) -> u64;
        fn youngest(&self) -> u64;

        fn size(&self) -> u64;
        fn shape(&self);        
    }
}

fn main() {
    // We create an aggregate here
    let mut fiba = ffi::create_fiba_with_sum();

    // do insert ops for 20 times
    for n in 1..=20u64 {
        fiba.pin_mut().insert(&n, &10);
    }
    // evict the first 10 elements
    for _ in 1..=10u64 {
        fiba.pin_mut().evict();
    }

    // general operations
    let oldest_key = fiba.pin_mut().oldest();
    let youngest_key = fiba.pin_mut().youngest();
    println!("oldest_key = {}, youngest_key = {}", oldest_key, youngest_key);

    // here the query op means aggregations for sum function
    // We have 10 keys and all values are 10, therefore it should return 100
    let aggregates = fiba.pin_mut().query();
    println!("aggregates = {}", aggregates);

    // other ops
    let size = fiba.pin_mut().size();
    println!("size = {}", size);

    // debug
    fiba.pin_mut().shape();
}
