#[allow(unused_imports)]
use std::fmt::Debug;
#[allow(unused_imports)]
use std::ops::BitOr;
#[allow(unused_imports)]
use std::mem;

#[allow(unused_macros)]
macro_rules! recurse {
    //($($pexpr:expr),*) => {{let z = ($($pexpr),*); z}}
    ($($pexpr:expr),*) => ({binding = ($($pexpr),*); 0})
}

macro_rules! recur_fn {
    ($fpointer:ident ($($pname:ident $colon:tt $type:ty),*) $arrow:tt $rtrn_type:ty $fbody:block) => 
        (fn $fpointer ($($pname : $type),*) -> $rtrn_type {
            let mut _memoize = ($($pname),*);
            let mut done = false;
            let mut outer_closure = move || while !done { // should be changed to a while loop...
                done = true;
                let recur = move |x|  {_memoize = x; done = false; 2.5};
                let inner_closure = Box::new(move |y| $fbody);
                inner_closure(_memoize);
            };
            outer_closure();
            fn recur ($($pname : $type),*) -> $rtrn_type $fbody
            let get_final_val = move |x| $fbody;
            get_final_val ((_memoize))
        })
   //($(fn $fpointer $gens:ty $fparams:tt $arrow:tt $rtrn_type:ty $fbody:block)) => $();
}

 recur_fn!{
    add_together (_num1 : u32, _num2 : u32) -> u32 {
       let sum = _num1 + _num2;
       if sum < 10 {
           println!("Okay");
           recur(_num1 + 1, _num2 + 1)
       } else {
            sum
       }
    }   
}

fn add_two (_num : u32) -> u32 {
       _num + 2
}

fn main() {
   println!("Tail recursive macro almost done!!!");
   println!("{:?}", add_together(0, 0));
}