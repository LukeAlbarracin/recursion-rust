use std::fmt::Debug;

macro_rules! tail_recur { //put a conditional within the macro
    ($name:ident $params:tt) => (recur ($name($params)))
}

macro_rules! recur_fn {
   //(fn $fpointer:ident $fparams:tt $arrow:tt $rtrn_type:ty $fbody:block) =>
   //     (fn $fpointer $fparams $arrow $rtrn_type {
   //            $fbody
   //     });
   //($fbody:block) => 
   ($(($pname:ident $colon:tt $type:ty)),*) => ($($type),*);
   ($fpointer:ident $fparams:tt $arrow:tt $rtrn_type:ty $fbody:block) =>
        (fn $fpointer $fparams $arrow $rtrn_type {
            let _memoize = $fparams;
            'fn_loop : loop {
                fn recur $fparams $arrow recur_fn!($fparams) {

                }
                //if foo == true else ...
            }
        });
   //($(fn $fpointer $gens:ty $fparams:tt $arrow:tt $rtrn_type:ty $fbody:block)) =>
   //     $();
    //_ => (panic!("Incorrectly formated function..."))
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

recur_fn!{
    add_two (_num : u32) -> u32 {
        _num + 2
    }
}

#[warn(dead_code)]
fn add_together (_num1 : u32, _num2 : u32) -> u32 {
    _num1 + _num2
}

fn main() {
   //println!("{:?}", add_two(5));
}