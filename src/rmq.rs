use ndarray::{array, Array, ArrayBase, Ix2, ShapeBuilder};
use std::cmp::min;

pub struct RMQ {
    pub sparse_table: Array<u32, Ix2>,
    pub values: Vec<u32>,
}

#[inline]
pub fn lg2(u: u32) -> u32 {
    u32::BITS - u.leading_zeros() - 1
}

impl RMQ {
    pub fn new(values: Vec<u32>) -> Self {
        let n = values.len();
        let k = lg2(n as u32) as usize;
        let mut st = Array::<u32, _>::zeros((k+1, n).f());
        for i in 0..n {
            st[[0, i]] = i as u32;
        }
        let mut j = 1;
        loop {
            if 1 << j <= n {
                let mut i = 0;
                loop {
                    if i + (1 << j) - 1 < n {
                        if values[st[[j - 1, i]] as usize]
                            < values[st[[j - 1, i + (1 << (j - 1))]] as usize]
                        {
                            st[[j, i]] = st[[j - 1, i]];
                        } else {
                            st[[j, i]] = st[[j - 1, i + (1 << (j - 1))]];
                        }
                    } else {
                        break;
                    }
                    i += 1;
                }
            } else {
                break;
            }
            j += 1;
        }
        Self {
            sparse_table: st,
            values,
        }
    }

    pub fn argmin(&self, l: u32, r: u32) -> usize {
        let j = lg2(r - l + 1) as usize;
        let ix1 = self.sparse_table[[j, l as usize]] as usize;
        let ix2 = self.sparse_table[[j, (r + 1 - (1 << j)) as usize]] as usize;
        let ix = if self.values[ix1] <= self.values[ix2] {
            ix1
        } else {
            ix2
        };
        ix
    }

    pub fn min(&self, l: u32, r: u32) -> u32 {
        let j = lg2(r - l + 1) as usize;
        let ix1 = self.sparse_table[[j, l as usize]] as usize;
        let ix2 = self.sparse_table[[j, (r + 1 - (1 << j)) as usize]] as usize;
        return min(self.values[ix1], self.values[ix2]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rmq() {
        let rmq = RMQ::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        // assert_eq!(rmq.min(0, 8), 1);
        // assert_eq!(rmq.min(0, 1), 1);
        // assert_eq!(rmq.min(3, 5), 4);
        // assert_eq!(rmq.argmin(0, 8),0);
        // assert_eq!(rmq.argmin(3, 5),3);
        assert_eq!(rmq.argmin(0, 1),0);
    }
}