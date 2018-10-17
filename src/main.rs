use std::any::Any;
use std::fmt::Debug;

/*  macro_rules! trampoline {
    ($x:expr) => (recur($x)) // Get unknown parameters and do more ...
} */

fn recur <T: 'static + Debug> (_params : T, _func : fn(T) -> T) -> T where T: Copy + PartialEq + ToString {
    let mut _memoize = _params;
        loop {
            let _temp = _func (_memoize);
            let foo = 8.to_string();
            println!("Debug: {:?}", &foo);
            if Some(&_temp.clone().to_string()) == Some(&foo) {
                println! ("Conditional complete !!!");
                break;
            } else {
                _memoize = _temp;
                println!("Value of temp : {:?}", &_temp);
                println! ("Value of Memoize : {:?}",  &_memoize);
            }
        }
    _memoize
}

fn add_two (_num : u32) -> u32 {
    _num + 2
}

fn main() {
   println!("{}", recur (0, add_two));
}
