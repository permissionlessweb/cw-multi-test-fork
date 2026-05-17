use crate::tests::default_app;
use crate::{no_init, AppBuilder, Executor, GovAcceptingModule};
use cosmwasm_std::Empty;
use crate::test_helpers::gov;
#[test]
fn default_gov() {
    let mut app = default_app();

    let creator_addr = app.api().addr_make("creator");
    let code = app.store_code_with_creator(creator_addr, gov::contract());

    let owner_addr = app.api().addr_make("owner");
    let contract = app
        .instantiate_contract(code, owner_addr.clone(), &Empty {}, &[], "govenius", None)
        .unwrap();

    app.execute_contract(owner_addr, contract, &Empty {}, &[])
        .unwrap_err();
}

#[test]
fn accepting_gov() {
    let mut app = AppBuilder::new()
        .with_gov(GovAcceptingModule::new())
        .build(no_init);

    let creator_addr = app.api().addr_make("creator");
    let code = app.store_code_with_creator(creator_addr, gov::contract());

    let owner_addr = app.api().addr_make("owner");
    let contract = app
        .instantiate_contract(code, owner_addr.clone(), &Empty {}, &[], "govenius", None)
        .unwrap();

    app.execute_contract(owner_addr, contract, &Empty {}, &[])
        .unwrap();
}
