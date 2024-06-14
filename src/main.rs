pub mod circular_buffer;
pub mod loops;
pub mod pointers;

pub mod myimpl;
pub fn main() {
    // println!("Main Function");
    // let gogogo = myimpl::Example {
    //     width: 30,
    //     height: 50,
    // };

    // gogogo.boo();
    // loops::demo_iter();
    // pointers::example();
    circular_buffer::main();
}
