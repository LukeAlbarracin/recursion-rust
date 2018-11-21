macro_rules! recur_fn {
    ($fpointer:ident ($($pname:ident $colon:tt $type:ty),*) -> $rtrn_type:ty $fbody:block) =>
        (fn $fpointer ($($pname : $type),*) -> $rtrn_type {
                let mut done = false;
                let mut _memoize = ($($pname),*);
                let mut _outer = move || loop {
                    println!("Hello there!");
                    done = true;
                    println!("Can you hear me?");
                        let mut _inner = move |_genp| {
                            println!("Inside this!");
                            let mut recur = move |x:u32,y:u32| {println!("Help is on its way!!!"); done = false; _memoize = (x,y); (x,y)};
                            let mut _super = move || {println!("inside super..."); $fbody};
                            _super();
                        };
                        println!("{:?}",_inner(_memoize));
                    if done == true {
                        println!("It's quite dark outside...");
                        break;
                    }
                };
                _outer();
            let recur = move |x,y| {println!("Help is on its way!!!"); done = false; _memoize = (x,y); (x,y)};
            //fn recur ($($pname:$type),*) -> $rtrn_type {101}
            let tail_func = |_params| { 
                $fbody
            };
            tail_func (_memoize)
        })
}

recur_fn!{
    add_together (_num1 : u32, _num2 : u32) -> u32 {
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
