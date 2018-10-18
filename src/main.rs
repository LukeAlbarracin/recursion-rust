use std::any::Any;
use std::io::Write;
use std::fmt::Debug;

//macro_rules! trampoline {
//   ($x:expr) => (println!("This is my project"))
//}

macro_rules! tail_recur {
    (fn $name:ident $params:tt $body:tt) => (recur($params, $name()))
}

// need to change unsized function (dynamically sized function) like &[T] -- use destructuring???
fn recur <T: 'static + Debug> (_params : T, _func : fn(T) -> T) -> T where T: Copy + PartialEq + ToString {
    let mut _memoize = _params;
        loop {
            let _temp = _func (_memoize);
            let foo = 8.to_string();
            println!("Debug: {:?}", &foo);
            if Some(&_temp.to_string()) == Some(&foo) {
                println!("Conditional complete !!!");
                _memoize = _temp;
                break;
            } else {
                _memoize = _temp;
                println!("Value of temp : {:?}", &_temp);
                println!("Value of Memoize : {:?}",  &_memoize);
            }
        }
    _memoize
}

fn add_two (_num : u32) -> u32 {
    _num + 2
}

fn main() {
   //trampoline!(0+0);
   //tail_recur!(add_two(0));
   //println!("{}", recur (0, add_two));
}