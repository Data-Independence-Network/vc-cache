
pub struct ByteCounts {
    pub currentBits: usize,
    pub currentByte: u8,
    pub data: Vec<u8>,
}

impl ByteCounts {

    pub fn new(
        capacity: usize
    ) -> ByteCounts {
        ByteCounts {
            data:  Vec::with_capacity(capacity),
            currentBits: 0,
            currentByte: 0,
        }
    }

    pub fn add4(mut self) {
        // don't add a value, to save on computation in highest data volume cases
        // also because value 4 is usually represented with a higher bit (which
        // is absent here)
        match self.currentBits {
            0 => {
                self.currentBits = 1;
            },
            1 => {
                self.currentBits = 2;
            },
            2 => {
                self.currentBits = 3;
            },
            3 => {
                self.data.push(self.currentByte);
                self.currentByte = 0;
                self.currentBits = 0;
            }
        }
    }

    pub fn add3(mut self) {
        match self.currentBits {
            0 => {
                self.currentByte &= 0b11000000;
                self.currentBits = 1;
            },
            1 => {
                self.currentByte &= 0b00110000;
                self.currentBits = 2;
            },
            2 => {
                self.currentByte &= 0b00001100;
                self.currentBits = 3;
            },
            3 => {
                self.currentByte &= 0b00000011;
                self.data.push(self.currentByte);
                self.currentByte = 0;
                self.currentBits = 0;
            }
        }
    }

    pub fn add2(mut self) {
        match self.currentBits {
            0 => {
                self.currentByte &= 0b10000000;
                self.currentBits = 1;
            },
            1 => {
                self.currentByte &= 0b00100000;
                self.currentBits = 2;
            },
            2 => {
                self.currentByte &= 0b00001000;
                self.currentBits = 3;
            },
            3 => {
                self.currentByte &= 0b00000010;
                self.data.push(self.currentByte);
                self.currentByte = 0;
                self.currentBits = 0;
            }
        }
    }

    pub fn add1(mut self) {
        match self.currentBits {
            0 => {
                self.currentByte &= 0b01000000;
                self.currentBits = 1;
            },
            1 => {
                self.currentByte &= 0b00010000;
                self.currentBits = 2;
            },
            2 => {
                self.currentByte &= 0b00000100;
                self.currentBits = 3;
            },
            3 => {
                self.currentByte &= 0b00000001;
                self.data.push(self.currentByte);
                self.currentByte = 0;
                self.currentBits = 0;
            }
        }
    }

    pub fn append(
        mut response: Vec<u8>
    ) {

    }
}