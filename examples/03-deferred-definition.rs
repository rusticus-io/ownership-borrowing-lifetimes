

struct A<'a> (&'a str);

impl<'a> Drop for A<'a> {
    fn drop(&mut self) {
        println!("drop {:?}", self.0);
    }
}


fn main() {
    {
        let x; // declaration           .....+  space reserved on the stack
                                            //  .
        {                                   //  .
            let a = A("a");                 //  .
            x = A("x"); // definition       // <!  lifetime of x starts here
        } // stack frame ends               //  !
        // x doesn't get dropped            //  !
        // drop of a                        //  !
        //  !
    } // stack frame ends                   //  !
    // drop of x                           <----+  lifetime or x ends here

}