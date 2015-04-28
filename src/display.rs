use std::fmt;

pub struct AStructInYoRust{
    pub zeroth: i32
}

impl fmt::Display for AStructInYoRust {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "YO SUM STRUCT IN DAT: {}", self.zeroth)
    }
}
