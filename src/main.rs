#[allow(unused_imports)]
use std::fmt::Debug;
use std::ops::BitOr;
use std::mem;


macro_rules! recur {
    //($($pexpr:expr),*) => {{let z = ($($pexpr),*); z}}
    ($($pexpr:expr),*) => {($($pexpr),*)}
    // remember that it is either return / changing outside environment
}

macro_rules! recur_fn {
    ($fpointer:ident ($($pname:ident $colon:tt $type:ty),*) $arrow:tt $rtrn_type:ty $fbody:block) => 
        (fn $fpointer ($($pname : $type),*) -> $rtrn_type {
            let mut _memoize = ($($pname),*);
            let mut done = false;
            let mut outer_closure = move |x| while !done { // should be changed to a while loop...
                done = true;
                let mut inner_closure = Box::new(|y| $fbody);
                inner_closure(_memoize);
            };
            outer_closure(_memoize);
            fn get_final_val ($($pname : $type),*) -> $rtrn_type $fbody
            get_final_val (5, 5)
        })
   //($(fn $fpointer $gens:ty $fparams:tt $arrow:tt $rtrn_type:ty $fbody:block)) => $();
}

 recur_fn!{
    add_together (_num1 : u32, _num2 : u32) -> u32 {
       let sum = _num1 + _num2;
       if sum < 10 {
           recur!(_num1 + 1, _num2 + 1)
       } else {
            150
       }
    }   
}

fn add_two (_num : u32) -> u32 {
       _num + 2
}

fn main() {
   println!("Tail recursive macro almost done!!!");
   println!("{:?}", add_together(5, 5));
}