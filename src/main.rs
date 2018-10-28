#[allow(unused_imports)]
use std::fmt::Debug;
use std::ops::BitOr;

macro_rules! recur {
    ($($expr:expr),*) => ({return ($($expr),*)})
}

macro_rules! recur_fn {
    ($fpointer:ident $fparams:tt $arrow:tt $rtrn_type:ty $fbody:block) => 
        (fn $fpointer $fparams $arrow $rtrn_type {
            let _memoize = recur_fn!(destructure_args $fparams);
            'fn_loop : loop {
                // can rloop have 0 arguments (impure function)???
                fn rloop $fparams $arrow recur_fn!(destructure_types $fparams) {
                    //multiple closures that take fbody (e.g. |x| = x + 1) POSSIBLY NOT MANDATORY???
                      let foo = Box::new(|x| $fbody); //should return a tuple of the updated arguments to be looped over...
                      let baz = foo(1);
                      (0,0)
                }
                break 'fn_loop;
                _memoize = rloop(0, 1);
            }
            fn get_final_val $fparams $arrow $rtrn_type {
                $fbody
            }
            get_final_val (0, 2)
        });
    //(fn $fpointer $fparams $arrow $rtrn_type {$fbody});
    (destructure_args ($($pname:ident $colon:tt $type:ty),*)) => (($($pname),*));
    (destructure_types ($($pname:ident $colon:tt $type:ty),*)) => (($($type),*))
   //($(fn $fpointer $gens:ty $fparams:tt $arrow:tt $rtrn_type:ty $fbody:block)) => $();
}

 recur_fn!{
    add_together (_num1 : u32, _num2 : u32) -> u32 {
    _num1 + _num2
    }   
}

fn add_two (_num : u32) -> u32 {
       _num + 2
}

fn main() {
   println!("Tail recursive macro almost done!!!");
   println!("{:?}", add_together(1, 4));
}