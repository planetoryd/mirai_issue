use mirai_annotations::verify;

use crate::Gen;



#[test]
fn boring() {
    verify!(false);
}


#[async_std::test]
async fn atest() {
    verify!(false);
}

#[test]
fn generic_struct() {
    let g = Gen(2);

    g.with_generics();
    verify!(false)
}

