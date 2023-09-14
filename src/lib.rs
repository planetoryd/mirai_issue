#[allow(unused)]

use contracts::*;
use mirai_annotations::*;

pub fn working() {
    verify!(false);
}

pub struct Empty;

impl Empty {
    pub fn without_generics() {
        verify!(false)
    }
}

pub struct Gen<T>(T);

impl<T> Gen<T> {
    pub fn with_generics() {
        verify!(false)
    }
}
