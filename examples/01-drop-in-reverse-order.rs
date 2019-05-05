use std::thread::sleep;
use std::time::Duration;

struct A<'a>(&'a str);

impl<'a> Drop for A<'a> {
    fn drop(&mut self) {
        println!("drop {:?}", self.0);
    }
}

fn main() { // main stack frame
    let _e1 = A("end 1");
    let _e2 = A("end 2");
    { // new stack frame
        let _x = A( "x" );
        let _y = A( "y" );
        let _z = A( "z" );
        { // new stack frame
            let _a = A( "a" );
            let _b = A( "b" );
            println!("wait 5 secs");
            sleep(Duration::from_secs(5));
            println!("before stack frame ends");
        } // stack frame ends
        // drop of b
        // drop of a
        println!("after stack frame ends");

        println!("wait 5 secs");
        sleep(Duration::from_secs(5));
        println!("before stack frame ends");
    } // stack frame ends
    // drop of z
    // drop of y
    // drop of x
    println!("after stack frame ends");

    println!("wait 5 secs");
    sleep(Duration::from_secs(5));
    println!("before main stack frame ends");
    println!("then drop of e1 and e2 in reverse order");
}
