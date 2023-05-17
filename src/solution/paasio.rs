use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    reader: R,
    reads: usize,
    bl: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            reader: wrapped,
            reads: 0,
            bl: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.bl
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let result = self.reader.read(buf);
        match result {
            Ok(r) => {
                self.bl += r;
                self.reads += 1;
            }
            Err(_) => {}
        }
        result
    }
}

pub struct WriteStats<W> {
    w: W,
    writesLen: usize,
    bl: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            w: wrapped,
            writesLen: 0,
            bl: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.w
    }

    pub fn bytes_through(&self) -> usize {
        self.bl
    }

    pub fn writes(&self) -> usize {
        self.writesLen
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let result = self.w.write(buf);
        match result {
            Ok(ws) => {
                self.bl += ws;
                self.writesLen += 1;
            }
            Err(_) => {}
        }
        result
    }

    fn flush(&mut self) -> Result<()> {
        self.w.flush()
    }
}
