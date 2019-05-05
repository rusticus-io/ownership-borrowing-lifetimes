
struct A<'a> (&'a str);

// implementing Drop
impl<'a> Drop for A<'a> {
    fn drop(&mut self) { // means this method is the destructor
        println!("drop {:?}", self.0);
    }
}

fn main() {
    let a = A("one single instance");
    let _b = a; // symbol a is invalidated in this line
    // println!("{:?}", a); // would produce an error

    let x = 3; // 3: i32 is a copy type
    let _y = x; // see associated diagram for explanation
                // x gets moved, but wait, the compiler remembers that x is a copy type
                // so we keep the symbol x
} // stack frame ends
// drop of _y
// drop of x
// drop of _b
// no drop of a because symbol a doesn't exist any more
