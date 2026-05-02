use crate::types::Binary;

pub(crate) trait Componant {
    fn load(&mut self, data: Binary);
    fn clear(&mut self);
}

pub(crate) trait Loader {
    fn load(&mut self, data: Binary);
}
