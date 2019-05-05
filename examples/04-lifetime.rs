

#[derive(Debug)]
struct A<'a> (&'a str);

impl<'a> Drop for A<'a> {
    fn drop(&mut self) {
        println!("drop {:?}", self.0);
    }
}


fn main() {
    {
        let x: &A; // declaration           ....+  space reserved on the stack (scope)
        //  .  x is now a reference !
        {                                   //  .
            let a = A("a");        // --+   //  .  lifetime of a starts here | # x lifetime doesn't
            x = &a; // definition       !   // <!  lifetime of x starts here | # cover a lifetime
            println!("{:?}", x);            //  !  usage of x is valid
        } // stack frame ends           !   //  !
        // x doesn't get dropped        !   //  !
        // drop of a               // <-+   //  !  lifetime of a ends here
                                            //  !
        println!("{:?}", x);                //  !  usage of x leads to borrow checker error
                                            //  !  if you comment out line 25 borrow checker doesn't
                                            //  !  complain (but should because of previous problem)
    } // stack frame ends                   //  !
    // drop of x                           <----+  lifetime of x ends here
}