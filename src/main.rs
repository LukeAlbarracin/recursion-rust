
#[allow(unused_imports)] 
use std::fmt::Debug;

#[allow(unused_macros)]
macro_rules! recur {
    ($($expr:expr),*) => ({return ($($expr),*)})
}

#[allow(unused_macros)]
macro_rules! recur_fn {
    //($($word:tt),*) => (if stringify!($word) == )
    (destructure_args $(($pname:ident $colon:tt $type:ty)),*) => ($($pname),*);
    (destructure_types $(($pname:ident $colon:tt $type:ty)),*) => ($($type),*);
    ($fpointer:ident $fparams:tt $arrow:tt $rtrn_type:ty $fbody:block) =>
        (fn $fpointer $fparams $arrow $rtrn_type {
            let _memoize = recur_fn!(destructure_args $fparams);
            'fn_loop : loop {
                fn rloop $fparams $arrow recur_fn!(destructure_types $fparams) {
                    // destructure into a let statement
                    // perform a condition check and get the function body of if/else statements (the control flow)
                    // function body ()
                    $fbody
                    break 'fn_loop;
                }
                //if foo  == true else ...
            }
        });
   //($(fn $fpointer $gens:ty $fparams:tt $arrow:tt $rtrn_type:ty $fbody:block)) => $();
}

/* recur_fn!{
    add_two (_num : u32) -> u32 {
        _num + 2
    }
}*/

#[allow(dead_code)] 
fn add_together (_num1 : u32, _num2 : u32) -> u32 {
    _num1 + _num2
}

fn main() {
   println!("Tail recursive macro almost done!!!");
   //println!("{:?}", add_two(5));
}