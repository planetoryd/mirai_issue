#[allow(unused)]

use contracts::*;
use mirai_annotations::*;

pub fn working() {
    verify!(false); // works
}

pub async fn lib_async() {
    verify!(false);
} 

pub struct Empty;

impl Empty {
    pub fn without_generics() {
        verify!(false) // works
    }
}

pub struct Gen<T>(T);

impl<T> Gen<T> {
    pub fn with_generics(&self) {
        verify!(false)
    }
}

#[test]
pub fn lib_test() {
    verify!(false);
}

#[cfg(test)]
mod test;