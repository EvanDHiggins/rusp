use std::io::Write;

pub enum IOStream {
    InMemoryBuffer(Vec<u8>),
    Stdout(std::io::Stdout),
}

impl IOStream {
    pub fn write(&mut self, buf: &[u8]) {
        match self {
            IOStream::InMemoryBuffer(vec) => {
                vec.extend_from_slice(buf);
            }
            IOStream::Stdout(stdout) => {
                stdout.write(buf).unwrap();
            }
        }
    }

    pub fn new_in_memory_buffer() -> IOStream {
        IOStream::InMemoryBuffer(Vec::new())
    }

    pub fn new_stdout() -> IOStream {
        IOStream::Stdout(std::io::stdout())
    }
}
