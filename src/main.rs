#![allow(unused_variables)]
#![recursion_limit="1000"]
macro_rules! recur_fn {
    ($fpointer:ident ($($pname:ident : $type:ty),*) -> $rtrn_type:ty $fbody:block) => 
        (fn $fpointer ($($pname : $type),*) -> $rtrn_type {
            let mut _memoize = ($($pname),*);
            fn recur ($($pname : $type),*) -> $rtrn_type $fbody
            let tail_val = move |_tailp| $fbody;
            tail_val (_memoize)
        })}

 recur_fn!{
   fibo_recursive (_num : u32) -> u32 {
       if _num == 0 || _num == 1 {
           _num
       } else {
          (recur(_num - 1) + recur(_num - 2))
       }
   }   
}

/* (defn fibo-recursive [n]
         (if (or (= n 0) (= n 1))
           n
           (+ (fibo-recursive (- n 1)) (fibo-recursive (- n 2))))) */



#[allow(dead_code)]
fn add_two (_num : u32) -> u32 {
    _num + 2
}

fn main() {
   println!("Tail recursive macro is complete!!!");
   println!("{:?}", fibo_recursive(8));
}