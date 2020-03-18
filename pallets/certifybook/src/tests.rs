// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Just a dummy test for the dummy function `do_something`
		// calling the `do_something` function with a value 42
		//assert_ok!(CertifybookModule::new_certificate(Origin::signed(1), 0x0000000000000000000000000000000000000000000000000000000000000001));
		// asserting that the stored value is equal to what we stored
		//assert_eq!(TemplateModule::something(), Some(42));
	});
}
