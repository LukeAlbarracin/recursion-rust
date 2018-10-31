#![allow(unused_variables)]
#![recursion_limit="1000"]
macro_rules! recur_fn {
    ($fpointer:ident ($($pname:ident : $type:ty),*) -> $rtrn_type:ty $fbody:block) => 
        (fn $fpointer ($($pname : $type),*) -> $rtrn_type {
            let mut _memoize = ($($pname),*);
            fn recur ($($pname : $type),*) -> $rtrn_type $fbody
            let tail_val = move |_tailp| $fbody;
            tail_val (_memoize)
        })
}

 recur_fn!{
    add_together (_num1 : u32, _num2 : u32) -> u32 {
       let sum = _num1 + _num2;
       if sum < 5 {
           println!("{}", sum);
           recur(recur(_num1 + 1, _num2 + 1), recur(_num1 + 1, _num2 + 1))
       } else {
            sum
       }
    }   
}

#[allow(dead_code)]
fn add_two (_num : u32) -> u32 {
    _num + 2
}

fn main() {
   println!("Tail recursive macro is complete!!!");
   println!("{:?}", add_together(0, 0));
}