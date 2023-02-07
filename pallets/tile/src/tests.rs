use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works() {
	new_test_ext().execute_with(|| {
		let tile_id = sp_core::H256([0; 32]);
		let codex = sp_core::H256([1; 32]);
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(Tile::new_tile(RuntimeOrigin::signed(1), tile_id, None, codex));
		// Read pallet storage and assert an expected result.
		assert_eq!(Tile::tile(tile_id), Some(crate::Tile { codex, parent: None }));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::NewTileDeclared { id: tile_id }.into());
	});
}
/*
#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			TemplateModule::cause_error(RuntimeOrigin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}
*/
