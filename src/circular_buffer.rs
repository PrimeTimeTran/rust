pub struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
    full: bool,
}

impl<T> CircularBuffer<T> {
    pub fn new(size: usize, default_value: T) -> Self
    where
        T: Clone,
    {
        let buffer = vec![Some(default_value); size]; // Initialize buffer with default values
        CircularBuffer {
            buffer,
            head: 0,
            tail: 0,
            size,
            full: false,
        }
    }

    pub fn write(&mut self, value: T) {
        if self.full {
            self.head = (self.head + 1) % self.size; // Move head to next slot if buffer is full
        }
        self.buffer[self.tail] = Some(value); // Write value to buffer at tail index
        self.tail = (self.tail + 1) % self.size; // Advance tail circularly
        self.full = self.tail == self.head; // Check if buffer is full after writing
    }

    pub fn read(&mut self) -> Option<T> {
        if self.head != self.tail || self.full {
            let value = self.buffer[self.head].take(); // Take ownership of the value
            self.head = (self.head + 1) % self.size; // Advance head circularly
            self.full = false; // Buffer is no longer full after reading
            value
        } else {
            None // Buffer is empty
        }
    }
}

pub fn main() {
    let mut buffer: CircularBuffer<i32> = CircularBuffer::new(5, 0);

    buffer.write(1);
    buffer.write(2);
    buffer.write(3);
    buffer.write(4);
    buffer.write(5);

    println!("Reading values:");
    while let Some(value) = buffer.read() {
        println!("{}", value);
    }
}
