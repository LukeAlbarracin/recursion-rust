macro_rules! recur_fn {
    (fn $fpointer:ident ($($pname:ident : $type:ty),*) -> $rtrn_type:ty $fbody:block) =>{
        fn $fpointer ($($pname : $type),*) -> $rtrn_type  {
            //fn recur ($($pname:$type),*)->($($type),*) {($($pname),*)}
            let _memoize = ($($pname),*);
            'outer : loop {
                fn recur ($($pname:$type),*)->($($type),*) {($($pname),*)}
                let foo = |x| {$fbody};
                match foo(_memoize) {
                    ($($pname),*) => {},
                    _ => {break 'outer;}
                }
                _memoize = foo;              
            }
            42
        }
    }
}
        
recur_fn!{
    fn add_together (_num1 : u32, _num2 : u32) -> u32 {
       let sum = _num1 + _num2;
       if sum < 10 {
           println!("This test ran okay!!!");
           recur(_num1+1, _num2+1)
       } else {
            sum
       }
    }   
}



fn main() {
   println!("Tail recursive macro almost done!!!");
   println!("{:?}", add_together(0, 0));
}
