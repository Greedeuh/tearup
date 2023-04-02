use tearup::{tearup, ContextCombinator};

use crate::helper::{
    FirstFullContext, FirstProof, FourthProof, SecondFullContext, SecondProof, ThirdProof,
};

#[test]
fn it_gives_access_to_variables() {
    it_uses_some_variables();
}

type Combination = ContextCombinator<FirstFullContext, SecondFullContext>;

#[tearup(Combination)]
fn it_uses_some_variables(mut a: FirstProof, b: SecondProof, c: ThirdProof, d: FourthProof) {
    assert_eq!(a.0, "first_proof");
    a.0 = "mutable".to_owned();
    assert_eq!(a.0, "mutable");
    assert_eq!(b.0, "second_proof");
    assert_eq!(c.0, "ref_to_first_proof");
    assert_eq!(d.0, "another_ref_to_second_proof");
}

#[cfg(feature = "async")]
mod asyncc {
    use tearup::{tearup, AsyncContextCombinator};

    use crate::helper::{
        AsyncFirstFullContext, AsyncSecondFullContext, FirstProof, FourthProof, SecondProof,
        ThirdProof,
    };

    #[tokio::test]
    async fn it_gives_access_to_variables() {
        it_uses_some_variables().await;
    }

    type Combination = AsyncContextCombinator<AsyncFirstFullContext, AsyncSecondFullContext>;

    #[tearup(Combination)]
    async fn it_uses_some_variables(
        mut a: FirstProof,
        b: SecondProof,
        c: ThirdProof,
        d: FourthProof,
    ) {
        assert_eq!(a.0, "first_proof");
        a.0 = "mutable".to_owned();
        assert_eq!(a.0, "mutable");
        assert_eq!(b.0, "second_proof");
        assert_eq!(c.0, "ref_to_first_proof");
        assert_eq!(d.0, "another_ref_to_second_proof");
    }
}
