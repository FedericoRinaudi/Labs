
pub struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    read_index: usize,
    write_index: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer{
            buffer: vec![None;capacity],
            capacity,
            read_index:0,
            write_index:0
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        match self.buffer[self.write_index] {
            Some(_) => Err(Error::FullBuffer),
            None => {
                self.buffer[self.write_index] = Some(_element);
                self.increasing_write_index();
                Ok(())
            }
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        match self.buffer[self.read_index].take() {     //take prende l'elemento se c'é e lascia None al suo posto
            Some(value) => {
                self.increasing_read_index();
                Ok(value)
            },
            None => Err(Error::EmptyBuffer)
        }
    }

    pub fn clear(&mut self) {
        self.buffer.iter_mut().for_each( |e| {e.take();} );
        self.read_index = 0;
        self.write_index = 0;
    }

    pub fn overwrite(&mut self, _element: T) {
        match self.buffer[self.write_index] {
            Some(_) => {
                self.buffer[self.write_index] = Some(_element);
                self.increasing_read_index();
                self.increasing_write_index();
            },
            None => {
                self.write(_element).unwrap();
            }
        }
    }

    fn increasing_read_index(&mut self){
        self.read_index += 1;
        if self.read_index == self.capacity {self.read_index=0;}
    }

    fn increasing_write_index(&mut self){
        self.write_index += 1;
        if self.write_index == self.capacity {self.write_index=0;}
    }

}
