
#[derive(Debug)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub enum ByteCode {
    LOAD_VAL(u8, u8), //DST, VAL
    KSTR(u8, u8),
    CALL(u8, u8, u8),
    RET0(u8, u8)
}

