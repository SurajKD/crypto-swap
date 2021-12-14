use crate::Decimal;
use anchor_lang::prelude::*;

const SIZE: u16 = 256; // UPDATE IN ARRAYS AS WELL!

#[account(zero_copy)]
#[derive(PartialEq)]
pub struct Oracle {
    pub data: [Record; 256],
    pub head: u16,
    pub amount: u16,
    pub size: u16,
}

#[zero_copy]
#[derive(PartialEq, Default, Debug)]
pub struct Record {
    pub timestamp: u64,
    pub price: Decimal,
}

impl Default for Oracle {
    fn default() -> Oracle {
        Oracle {
            data: [Record::default(); 256],
            head: SIZE - 1,
            amount: 0,
            size: SIZE,
        }
    }
}

impl Oracle {
    pub fn add_record(self: &mut Self, timestamp: u64, price: Decimal) {
        let record = Record { timestamp, price };

        self.head = (self.head + 1) % self.size;
        self.data[self.head as usize] = record;

        if self.amount < self.size {
            self.amount += 1;
        }
    }

    pub fn init(self: &mut Self) {
        self.size = SIZE;
        self.head = SIZE - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_recording() {
        let mut oracle = Oracle::default();
        assert_eq!({ oracle.size }, SIZE);

        let mut index: u64 = 0;

        // fill
        while index < SIZE as u64 {
            oracle.add_record(index, Decimal { v: index as u128 });

            assert_eq!(oracle.head as u64, index);
            assert_eq!(oracle.amount as u64, index + 1);
            assert_eq!({ oracle.data[oracle.head as usize].timestamp }, index);
            assert_eq!(oracle.data[oracle.head as usize].price.v, index as u128);

            index += 1;
        }

        // second fill
        while index < 2 * SIZE as u64 {
            oracle.add_record(index, Decimal { v: index as u128 });

            assert_eq!(oracle.head as u64, index - SIZE as u64);
            assert_eq!(oracle.amount as u64, SIZE as u64);
            assert_eq!({ oracle.data[oracle.head as usize].timestamp }, index);
            assert_eq!(oracle.data[oracle.head as usize].price.v, index as u128);

            index += 1;
        }
    }
}