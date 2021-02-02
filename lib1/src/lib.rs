pub trait Serde {
    fn serialize(&self) -> &[u8];
    fn deserialize<'a>(buffer: &'a [u8]) -> &'a Self;
}
