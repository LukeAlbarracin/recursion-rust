#[allow(unused_imports)]
use std::fmt::Debug;
use std::ops::BitOr;

macro_rules! recur {
    ($($pexpr:expr),*) => {_memoize = ($($pexpr),*)}
}

macro_rules! recur_fn {
    ($fpointer:ident ($($pname:ident $colon:tt $type:ty),*) $arrow:tt $rtrn_type:ty $fbody:block) => 
        (fn $fpointer ($($pname $colon $type),*) $arrow $rtrn_type {
            let _memoize = ($($pname),*);
            'fn_loop : loop {
                // can rloop have 0 arguments (impure function)???
                fn rloop ($($pname $colon $type),*) $arrow ($($type),*) {
                    //multiple closures that take fbody (e.g. |x| = x + 1) POSSIBLY NOT MANDATORY???
                      let foo = Box::new(|x| $fbody); //should return a tuple of the updated arguments to be looped over...
                      let baz = foo(0);
                      return baz
                      (0,0)
                }
                break 'fn_loop;
                _memoize = rloop(0, 1);
            }
            fn get_final_val ($($pname $colon $type),*) $arrow $rtrn_type $fbody
            get_final_val (1, 2)
        });
    //(destructure_args ($($pname:ident $colon:tt $type:ty),*)) => (($($pname),*));
    //(destructure_types ($($pname:ident $colon:tt $type:ty),*)) => (($($type),*))
   //($(fn $fpointer $gens:ty $fparams:tt $arrow:tt $rtrn_type:ty $fbody:block)) => $();
}

 recur_fn!{
    add_together (_num1 : u32, _num2 : u32) -> u32 {
       let sum = _num1 + _num2;
       if sum < 10 {
           recur!(_num1 + 1, _num2 + 1);
           return sum
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
   println!("{:?}", add_together(1, 4));
}