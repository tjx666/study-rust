mod common;

use common::setup;
use tests::add;

#[test]
fn it_test() {
    setup();
    assert_eq!(add(1, 2), 3);
}
