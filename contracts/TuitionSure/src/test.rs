#![cfg(test)]

use crate::{TuitionSure, TuitionSureClient};
use soroban_sdk::{
    symbol_short,
    testutils::Address as _,
    token::{StellarAssetClient, TokenClient},
    Address, Env,
};

fn create_token(env: &Env, admin: &Address) -> (Address, TokenClient, StellarAssetClient) {
    let sac = env.register_stellar_asset_contract_v2(admin.clone());
    let token_address = sac.address();

    (
        token_address.clone(),
        TokenClient::new(env, &token_address),
        StellarAssetClient::new(env, &token_address),
    )
}

#[test]
fn happy_path_invoice_is_paid_end_to_end() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TuitionSure, ());
    let client = TuitionSureClient::new(&env, &contract_id);

    let school = Address::generate(&env);
    let student = Address::generate(&env);
    let parent = Address::generate(&env);
    let admin = Address::generate(&env);

    let (token_address, token, asset) = create_token(&env, &admin);
    asset.mint(&parent, &1_000);

    client.create_invoice(
        &symbol_short!("MIDTERM"),
        &student,
        &school,
        &token_address,
        &750,
    );

    client.confirm_invoice(&symbol_short!("MIDTERM"));
    client.pay_invoice(&symbol_short!("MIDTERM"), &parent);

    assert_eq!(token.balance(&school), 750);
    assert_eq!(token.balance(&parent), 250);
}

#[test]
#[should_panic(expected = "invoice already exists")]
fn edge_case_duplicate_invoice_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TuitionSure, ());
    let client = TuitionSureClient::new(&env, &contract_id);

    let school = Address::generate(&env);
    let student = Address::generate(&env);
    let admin = Address::generate(&env);
    let (token_address, _, _) = create_token(&env, &admin);

    client.create_invoice(
        &symbol_short!("TUITION"),
        &student,
        &school,
        &token_address,
        &500,
    );

    client.create_invoice(
        &symbol_short!("TUITION"),
        &student,
        &school,
        &token_address,
        &500,
    );
}

#[test]
fn state_verification_after_payment() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TuitionSure, ());
    let client = TuitionSureClient::new(&env, &contract_id);

    let school = Address::generate(&env);
    let student = Address::generate(&env);
    let parent = Address::generate(&env);
    let admin = Address::generate(&env);

    let (token_address, _, asset) = create_token(&env, &admin);
    asset.mint(&parent, &900);

    client.create_invoice(
        &symbol_short!("PRELIM"),
        &student,
        &school,
        &token_address,
        &900,
    );

    client.confirm_invoice(&symbol_short!("PRELIM"));
    client.pay_invoice(&symbol_short!("PRELIM"), &parent);

    let invoice = client.get_invoice(&symbol_short!("PRELIM"));

    assert_eq!(invoice.student, student);
    assert_eq!(invoice.school, school);
    assert_eq!(invoice.amount, 900);
    assert_eq!(invoice.confirmed, true);
    assert_eq!(invoice.paid, true);
    assert_eq!(invoice.payer, Some(parent));
}

#[test]
fn balance_due_is_zero_after_payment() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TuitionSure, ());
    let client = TuitionSureClient::new(&env, &contract_id);

    let school = Address::generate(&env);
    let student = Address::generate(&env);
    let parent = Address::generate(&env);
    let admin = Address::generate(&env);

    let (token_address, _, asset) = create_token(&env, &admin);
    asset.mint(&parent, &1_200);

    client.create_invoice(
        &symbol_short!("LABFEE"),
        &student,
        &school,
        &token_address,
        &1_200,
    );

    assert_eq!(client.balance_due(&symbol_short!("LABFEE")), 1_200);

    client.confirm_invoice(&symbol_short!("LABFEE"));
    client.pay_invoice(&symbol_short!("LABFEE"), &parent);

    assert_eq!(client.balance_due(&symbol_short!("LABFEE")), 0);
}

#[test]
#[should_panic(expected = "student has not confirmed invoice")]
fn payment_before_student_confirmation_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TuitionSure, ());
    let client = TuitionSureClient::new(&env, &contract_id);

    let school = Address::generate(&env);
    let student = Address::generate(&env);
    let parent = Address::generate(&env);
    let admin = Address::generate(&env);

    let (token_address, _, asset) = create_token(&env, &admin);
    asset.mint(&parent, &700);

    client.create_invoice(
        &symbol_short!("EXAM"),
        &student,
        &school,
        &token_address,
        &700,
    );

    client.pay_invoice(&symbol_short!("EXAM"), &parent);
}