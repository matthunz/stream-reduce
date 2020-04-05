use futures::executor::block_on;
use futures::stream;
use stream_reduce::Reduce;

fn sum(v: Vec<u8>) -> Option<u8> {
    block_on(stream::iter(v).reduce(|a, b| async move { a + b }))
}

#[test]
fn test_reduce_some() {
    let v = vec![1, 2, 3, 4, 5];
    assert_eq!(Some(15), sum(v));
}

#[test]
fn test_reduce_one() {
    let v = vec![1];
    assert_eq!(Some(1), sum(v));
}

#[test]
fn test_reduce_none() {
    let v = Vec::new();
    assert_eq!(None, sum(v));
}
