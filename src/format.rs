/// nothing ⇒ Display
/// ? ⇒ Debug
/// o ⇒ Octal
/// x ⇒ LowerHex
/// X ⇒ UpperHex
/// p ⇒ Pointer
/// b ⇒ Binary
/// e ⇒ LowerExp
/// E ⇒ UpperExp
/// evaluate for traits implementation
#[derive(Copy, Clone, Debug)]
pub enum Format {
    /// octal format
    Octal,
    /// lower hex format
    LowerHex,
    /// upper hex format
    UpperHex,
    /// pointer format
    Pointer,
    /// binary format
    Binary,
    /// lower exp format
    LowerExp,
    /// upper exp format
    UpperExp,
    /// unknown format
    Unknown,
}

impl Format {
    /// Formats a given u8 according to the base Format
    ///
    /// # Arguments
    ///
    /// * `data` - The byte to be formatted
    /// * `prefix` - whether or not to add a prefix
    pub fn format(&self, data: u8, prefix: bool) -> String {
        if prefix {
            match &self {
                Self::Octal => format!("{:#06o}", data),
                Self::LowerHex => format!("{:#04x}", data),
                Self::UpperHex => format!("{:#04X}", data),
                Self::Binary => format!("{:#010b}", data),
                _ => panic!("format is not implemented for this Format"),
            }
            .to_string()
        } else {
            match &self {
                Self::Octal => format!("{:04o}", data),
                Self::LowerHex => format!("{:02x}", data),
                Self::UpperHex => format!("{:02X}", data),
                Self::Binary => format!("{:08b}", data),
                _ => panic!("format is not implemented for this Format"),
            }
            .to_string()
        }
    }
}
