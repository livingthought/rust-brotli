use core;

#[derive(Clone, Debug,Copy)]
pub struct InputPair<'a>(pub &'a [u8],pub &'a [u8]);

impl<'a> core::cmp::PartialEq for InputPair<'a> {
    fn eq<'b>(&self, other: &InputPair<'b>) -> bool {
        if self.0.len() + self.1.len() != other.0.len() + other.1.len() {
            return false;
        }
        for (a_iter, b_iter) in self.0.iter().chain(self.1.iter()).zip(other.0.iter().chain(other.1.iter())) {
            if *a_iter != *b_iter {
                return false;
            }
        }
        return true;
    }
}
impl<'a> core::ops::Index<usize> for InputPair<'a> {
  type Output = u8;
  fn index(&self, index:usize) -> &u8 {
    if index >= self.0.len() {
      &self.1[index - self.0.len()]
    } else {
      &self.0[index]
    }
  }
}
impl<'a> core::fmt::LowerHex for InputPair<'a> {
    fn fmt(&self, fmtr: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        for item in self.0 {
            try!( fmtr.write_fmt(format_args!("{:02x}", item)));
        }
        for item in self.1 {
            try!( fmtr.write_fmt(format_args!("{:02x}", item)));
        }
        Ok(())
    }
}

impl<'a> InputPair<'a> {
    pub fn split_at(&self, loc : usize) -> (InputPair<'a>, InputPair<'a>) {
        if loc >= self.0.len() {
            let (first, second) = self.1.split_at(core::cmp::min(loc - self.0.len(),
                                                                 self.1.len()));
            return (InputPair::<'a>(self.0, first), InputPair::<'a>(&[], second));
        }
        let (first, second) = self.0.split_at(core::cmp::min(loc,
                                                             self.0.len()));
        (InputPair::<'a>(first, &[]), InputPair::<'a>(second, self.1))
    }
    pub fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }
}
