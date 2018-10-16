use std::any::Any;
use std::fmt::Debug;

/*  macro_rules! trampoline {
    ($x:expr) => (recur($x)) // Get unknown parameters and do more ...
} */

fn recur <T: 'static + Debug> (_params : T, _func : &Fn(T) -> T) -> T where T: Copy + PartialEq {
    let mut _memoize = _params;
        loop {
            let _temp = _func (_func (_memoize));
            let foo : &Any = &12;
            if Some(&_temp) == foo.downcast_ref::<T>() {
                println! ("Conditional complete !!!");
                break;
            } else {
                println! ("{:?}",  &_memoize);
                _memoize = _temp.clone();
            }
        }
    _memoize
}

fn add_two (_num : u32) -> u32 {
    _num + 2
}

fn main() {
   println!("{}", recur (6, &add_two));
}
