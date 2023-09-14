#[allow(unused)]

use contracts::*;
use mirai_annotations::*;

pub fn working() {
    verify!(false);
}

struct Empty;

impl Empty {
    fn without_generics() {
        verify!(false)
    }
}

struct Gen<T>(T);

impl<T> Gen<T> {
    fn with_generics() {
        verify!(false)
    }
}
