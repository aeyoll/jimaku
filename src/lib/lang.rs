pub trait Lang {
    type Err;

    fn get_lang(&self) -> &'static str;

    fn from_code(s: &str) -> Result<Self, Self::Err>
    where
        Self: Sized;
}
