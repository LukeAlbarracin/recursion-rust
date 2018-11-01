#![allow(dead_code)]
macro_rules! recur_fn {
    (fn $fpointer:ident ($($pname:ident : $type:ty),*) -> $rtrn_type:ty $fbody:block) => 
        (fn $fpointer ($($pname : $type),*) -> $rtrn_type {
            fn recur ($($pname : $type),*) -> $rtrn_type $fbody
            let tail_val = move |_tailp| $fbody;
            tail_val ($($pname),*)
        });
    ($item:item) => ($item)
}

recur_fn! {
    fn fibonacci_recur (_num : u32) -> u32 {
        if _num == 0 || _num == 1 {
           _num
       } else {
          fibonacci_recur(_num - 1) + fibonacci_recur(_num - 2)
       }
    }
}

recur_fn! {
    fn recursive_iteration (_num : u32) -> u32 {
        if _num < 4 {
            recur (_num + 1)
        } else {
            _num
        }
    }
}

recur_fn! {
    fn print_something() {
        println!("This does absolutely nothing except print this...");
    }
}


fn main() {
   println!("Hello, Tail Recursive Macro!");
   assert_eq!(8, fibonacci_recur(6)); // Fibonacci sequence : 1,1,2,3,5,8,13,21...
   assert_eq!(4, recursive_iteration(0));
   print_something(); 
   
}