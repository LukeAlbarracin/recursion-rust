# recursion-rust

Simply put ... a tiny macro can be used for a tail recursive call but I added the recur keyword that is specific to the macro within my project...

Look at the below code :

  macro_rules! recur_fn {($item:item)=>($item)}
  
 
Now by wrapping my code within the macro :
  
  recur_fn! { 
    fn fibonacci_recur (_num : u32) -> u32 {
        if _num == 0 || _num == 1 {
           _num
       } else {
          fibonacci_recur(_num - 1) + fibonacci_recur(_num - 2)
       }
    }
  }

Instead of getting a stack overflow error or an error that forbids a tail call, we are allowed to call the function
recursively. This is because of the hygienic macro structure of rust. In order to avoid confusion amongst identifiers it often assigns a new identifier to the function pointer, resulting in the macro wrapping itself like foobar(foobar(foobar())). Yes, this is not pure tail call optimization but it is the closest way to replicate a tail call without a stack overflow error.

