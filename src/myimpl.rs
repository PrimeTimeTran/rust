pub struct Example {
    pub width: u32,
    pub height: u32,
}

impl Example {
    pub fn boo(&self) {
        println!("boo! Example::boo() was called!");
    }
}
