use frame_support::{assert_noop, assert_ok, error::BadOrigin};

use super::*;
use mock::{Event, *};

type NFTModule = Pallet<Test>;

#[test]
fn create_class_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NFTModule::create_class(
			Origin::signed(ALICE),
			"some metadata about token".as_bytes().to_vec(),
			ClassData { is_pool: true },
			TEST_PRICE
		));
		let event = Event::NFT(crate::Event::NFTTokenClassCreated(ALICE, CLASS_ID));
		assert_eq!(last_event(), event);
	})
}

#[test]
fn create_class_fails() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			NFTModule::create_class(
				Origin::none(),
				"some metadata about token".as_bytes().to_vec(),
				ClassData { is_pool: true },
				TEST_PRICE
			),
			BadOrigin
		);
	})
}

#[test]
fn mint_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NFTModule::create_class(
			Origin::signed(ALICE),
			"some metadata about token".as_bytes().to_vec(),
			ClassData { is_pool: true },
			TEST_PRICE
		));
		let event = Event::NFT(crate::Event::NFTTokenClassCreated(ALICE, CLASS_ID));
		assert_eq!(last_event(), event);

		assert_ok!(NFTModule::mint(
			Origin::signed(ALICE),
			0,
			"a token".as_bytes().to_vec(),
			TokenData {
				locked: false,
				emote: EMOTE.as_bytes().to_vec()
			},
			TEST_QUANTITY,
		));
		let event = Event::NFT(crate::Event::NFTTokenMinted(ALICE, CLASS_ID, TEST_QUANTITY));
		assert_eq!(last_event(), event);
	});
}

#[test]
fn mint_fails() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NFTModule::create_class(
			Origin::signed(ALICE),
			"some metadata about token".as_bytes().to_vec(),
			ClassData { is_pool: true },
			TEST_PRICE
		));
		let event = Event::NFT(crate::Event::NFTTokenClassCreated(ALICE, CLASS_ID));
		assert_eq!(last_event(), event);

		assert_noop!(
			NFTModule::mint(
				Origin::signed(BOB),
				0,
				"a token".as_bytes().to_vec(),
				TokenData {
					locked: false,
					emote: EMOTE.as_bytes().to_vec()
				},
				TEST_QUANTITY,
			),
			Error::<Test>::NotClassOwner
		);
	});
}

#[test]
fn transfer_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NFTModule::create_class(
			Origin::signed(ALICE),
			"some metadata about token".as_bytes().to_vec(),
			ClassData { is_pool: true },
			TEST_PRICE
		));

		assert_ok!(NFTModule::mint(
			Origin::signed(ALICE),
			0,
			"a token".as_bytes().to_vec(),
			TokenData {
				locked: false,
				emote: EMOTE.as_bytes().to_vec()
			},
			TEST_QUANTITY,
		));

		assert_ok!(NFTModule::transfer(Origin::signed(ALICE), BOB, (CLASS_ID, TOKEN_ID)));
		let event = Event::NFT(crate::Event::NFTTokenTransferred(ALICE, BOB, CLASS_ID, TOKEN_ID));
		assert_eq!(last_event(), event);
	});
}

#[test]
fn transfer_fails() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NFTModule::create_class(
			Origin::signed(ALICE),
			"some metadata about token".as_bytes().to_vec(),
			ClassData { is_pool: true },
			TEST_PRICE
		));

		assert_ok!(NFTModule::mint(
			Origin::signed(ALICE),
			0,
			"a token".as_bytes().to_vec(),
			TokenData {
				locked: false,
				emote: EMOTE.as_bytes().to_vec()
			},
			TEST_QUANTITY,
		));

		assert_noop!(
			NFTModule::transfer(Origin::signed(BOB), ALICE, (CLASS_ID, TOKEN_ID)),
			Error::<Test>::NotTokenOwner
		);

		assert_noop!(
			NFTModule::transfer(Origin::signed(ALICE), ALICE, (CLASS_ID, TOKEN_ID)),
			Error::<Test>::CannotSendToSelf
		);
	});
}

#[test]
fn burn_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NFTModule::create_class(
			Origin::signed(ALICE),
			"some metadata about token".as_bytes().to_vec(),
			ClassData { is_pool: true },
			TEST_PRICE
		));

		assert_ok!(NFTModule::mint(
			Origin::signed(ALICE),
			0,
			"a token".as_bytes().to_vec(),
			TokenData {
				locked: false,
				emote: EMOTE.as_bytes().to_vec()
			},
			TEST_QUANTITY,
		));

		assert_ok!(NFTModule::burn(Origin::signed(ALICE), (CLASS_ID, TOKEN_ID)));
		let event = Event::NFT(crate::Event::NFTTokenBurned(ALICE, CLASS_ID, TOKEN_ID));
		assert_eq!(last_event(), event);
	});
}

#[test]
fn burn_fails() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NFTModule::create_class(
			Origin::signed(ALICE),
			"some metadata about token".as_bytes().to_vec(),
			ClassData { is_pool: true },
			TEST_PRICE
		));

		assert_ok!(NFTModule::mint(
			Origin::signed(ALICE),
			0,
			"a token".as_bytes().to_vec(),
			TokenData {
				locked: false,
				emote: EMOTE.as_bytes().to_vec()
			},
			TEST_QUANTITY,
		));

		assert_noop!(
			NFTModule::burn(Origin::signed(BOB), (CLASS_ID, TOKEN_ID)),
			Error::<Test>::NotTokenOwner
		);
	});
}

#[test]
fn destroy_class_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NFTModule::create_class(
			Origin::signed(ALICE),
			"some metadata about token".as_bytes().to_vec(),
			ClassData { is_pool: true },
			TEST_PRICE
		));

		assert_ok!(NFTModule::destroy_class(Origin::signed(ALICE), CLASS_ID));
	});
}

#[test]
fn destroy_class_fails() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NFTModule::create_class(
			Origin::signed(ALICE),
			"some metadata about token".as_bytes().to_vec(),
			ClassData { is_pool: true },
			TEST_PRICE
		));

		assert_ok!(NFTModule::mint(
			Origin::signed(ALICE),
			0,
			"a token".as_bytes().to_vec(),
			TokenData {
				locked: false,
				emote: EMOTE.as_bytes().to_vec()
			},
			TEST_QUANTITY,
		));

		assert_noop!(
			NFTModule::destroy_class(Origin::signed(ALICE), CLASS_ID),
			Error::<Test>::NonZeroIssuance
		);
	});
}

#[test]
fn toggle_lock_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NFTModule::create_class(
			Origin::signed(ALICE),
			"some metadata about token".as_bytes().to_vec(),
			ClassData { is_pool: true },
			TEST_PRICE
		));

		assert_ok!(NFTModule::mint(
			Origin::signed(ALICE),
			0,
			"a token".as_bytes().to_vec(),
			TokenData {
				locked: false,
				emote: EMOTE.as_bytes().to_vec()
			},
			TEST_QUANTITY,
		));

		assert_ok!(NFTModule::toggle_lock(&ALICE, (CLASS_ID, TOKEN_ID)));
		let locked = NFTModule::is_locked((CLASS_ID, TOKEN_ID));
		assert!(locked.unwrap());
	});
}

#[test]
fn toggle_lock_fails() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NFTModule::create_class(
			Origin::signed(ALICE),
			"some metadata about token".as_bytes().to_vec(),
			ClassData { is_pool: true },
			TEST_PRICE
		));

		assert_ok!(NFTModule::mint(
			Origin::signed(ALICE),
			0,
			"a token".as_bytes().to_vec(),
			TokenData {
				locked: false,
				emote: EMOTE.as_bytes().to_vec()
			},
			TEST_QUANTITY,
		));

		assert_noop!(
			NFTModule::toggle_lock(&BOB, (CLASS_ID, TOKEN_ID)),
			Error::<Test>::NotTokenOwner
		);
	});
}

#[test]
fn buy_from_pool_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NFTModule::create_class(
			Origin::signed(ALICE),
			"some metadata about token".as_bytes().to_vec(),
			ClassData { is_pool: true },
			TEST_PRICE
		));

		assert_ok!(NFTModule::mint(
			Origin::signed(ALICE),
			0,
			"a token".as_bytes().to_vec(),
			TokenData {
				locked: false,
				emote: EMOTE.as_bytes().to_vec()
			},
			TEST_QUANTITY,
		));

		assert_ok!(NFTModule::buy_from_pool(Origin::signed(BOB), (CLASS_ID, TOKEN_ID)));
		let event = Event::NFT(crate::Event::NFTBoughtFromPool(ALICE, BOB, CLASS_ID, TOKEN_ID));
		assert_eq!(last_event(), event);
	});
}

#[test]
fn buy_from_pool_fails() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NFTModule::create_class(
			Origin::signed(ALICE),
			"some metadata about token".as_bytes().to_vec(),
			ClassData { is_pool: true },
			TEST_PRICE
		));

		assert_ok!(NFTModule::mint(
			Origin::signed(ALICE),
			0,
			"a token".as_bytes().to_vec(),
			TokenData {
				locked: false,
				emote: EMOTE.as_bytes().to_vec()
			},
			TEST_QUANTITY,
		));

		assert_ok!(NFTModule::transfer(Origin::signed(ALICE), BOB, (CLASS_ID, TOKEN_ID)));

		assert_noop!(
			NFTModule::buy_from_pool(Origin::signed(ALICE), (CLASS_ID, TOKEN_ID)),
			Error::<Test>::TokenAlreadyHasAnOwner
		);

		assert_ok!(NFTModule::transfer(Origin::signed(BOB), ALICE, (CLASS_ID, TOKEN_ID)));

		assert_noop!(
			NFTModule::buy_from_pool(Origin::signed(ALICE), (CLASS_ID, TOKEN_ID)),
			Error::<Test>::CannotBuyOwnToken
		);
	});
}

#[test]
fn buy_from_pool_fails_notapool() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NFTModule::create_class(
			Origin::signed(ALICE),
			"some metadata about token".as_bytes().to_vec(),
			ClassData { is_pool: false },
			TEST_PRICE
		));

		assert_ok!(NFTModule::mint(
			Origin::signed(ALICE),
			0,
			"a token".as_bytes().to_vec(),
			TokenData {
				locked: false,
				emote: EMOTE.as_bytes().to_vec()
			},
			TEST_QUANTITY,
		));

		assert_noop!(
			NFTModule::buy_from_pool(Origin::signed(BOB), (CLASS_ID, TOKEN_ID)),
			Error::<Test>::NotAPool
		);
	});
}

#[test]
fn sell_to_pool_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NFTModule::create_class(
			Origin::signed(ALICE),
			"some metadata about token".as_bytes().to_vec(),
			ClassData { is_pool: true },
			TEST_PRICE
		));

		assert_ok!(NFTModule::mint(
			Origin::signed(ALICE),
			0,
			"a token".as_bytes().to_vec(),
			TokenData {
				locked: false,
				emote: EMOTE.as_bytes().to_vec()
			},
			TEST_QUANTITY,
		));

		assert_ok!(NFTModule::transfer(Origin::signed(ALICE), BOB, (CLASS_ID, TOKEN_ID)));

		assert_ok!(NFTModule::sell_to_pool(Origin::signed(BOB), (CLASS_ID, TOKEN_ID)));
		let event = Event::NFT(crate::Event::NFTSoldToPool(BOB, ALICE, CLASS_ID, TOKEN_ID));
		assert_eq!(last_event(), event);
	});
}

#[test]
fn sell_to_pool_fails() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NFTModule::create_class(
			Origin::signed(ALICE),
			"some metadata about token".as_bytes().to_vec(),
			ClassData { is_pool: true },
			TEST_PRICE
		));

		assert_ok!(NFTModule::mint(
			Origin::signed(ALICE),
			0,
			"a token".as_bytes().to_vec(),
			TokenData {
				locked: false,
				emote: EMOTE.as_bytes().to_vec()
			},
			TEST_QUANTITY,
		));

		assert_noop!(
			NFTModule::sell_to_pool(Origin::signed(ALICE), (CLASS_ID, TOKEN_ID)),
			Error::<Test>::CannotSellPoolToken
		);

		assert_ok!(NFTModule::transfer(
			Origin::signed(ALICE),
			CHARLIE,
			(CLASS_ID, TOKEN_ID)
		));

		assert_noop!(
			NFTModule::sell_to_pool(Origin::signed(BOB), (CLASS_ID, TOKEN_ID)),
			Error::<Test>::NotTokenOwner
		);
	});
}
