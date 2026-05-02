#[derive(Debug, Clone, Copy)]
pub(crate) struct Address(pub(crate) u16);

impl Address {
    pub(crate) fn zero() -> Self {
        Self(0u16)
    }

    pub(crate) fn new(adr: u16) -> Self {
        Self(adr)
    }
}
