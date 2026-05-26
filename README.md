# TuitionSure

TuitionSure is a Stellar-based tuition payment verifier that makes sure school money reaches the verified school wallet, not a student's personal spending wallet.

## Live Testnet Contract

**Contract ID:** `CBIKU4733GSN52GTAYFA2JT2RIDTB3VTQMTBCNRMWEJ2YHZQISV2B7UP`

**Explorer:** [View on Stellar Expert](https://stellar.expert/explorer/testnet/contract/CBIKU4733GSN52GTAYFA2JT2RIDTB3VTQMTBCNRMWEJ2YHZQISV2B7UP)

## Screenshot

<img width="1920" height="945" alt="TuitionSure screenshot" src="https://github.com/user-attachments/assets/2a0b2a55-8cdb-490b-a753-4528418ef0d9" />

## Problem

An OFW parent in Dubai sends PHP 18,000 tuition money to their college student in Laguna, but the money can be delayed, misused, or mixed with allowance, causing missed enrollment validation or exam permit release.

## Solution

TuitionSure lets the school create a tuition invoice, lets the student confirm the invoice, and lets the parent pay through Stellar USDC while a Soroban smart contract records the payment only for the verified school invoice and school wallet.

## Core MVP Flow

School creates invoice -> Student confirms invoice -> Parent pays invoice -> Soroban records payment -> School verifies tuition status.

## Target Users

- Filipino senior high, college, and vocational students who need tuition, lab fee, or exam permit payments verified quickly.
- OFW parents, guardians, relatives, churches, and scholarship sponsors who want tuition support to go directly to school fees.
- Small and mid-sized Philippine schools that still reconcile payments through bank screenshots, cashier queues, and spreadsheets.

## Timeline

- Bootcamp ideation and MVP design
- Soroban smart contract implementation
- Local testing with exactly 5 tests
- Stellar testnet deployment
- GitHub and Rise In submission

## Stellar Features Used

- USDC transfers
- Soroban smart contracts
- Trustlines

## Vision and Purpose

TuitionSure helps Filipino students, parents, OFW guardians, scholarship sponsors, and schools coordinate tuition payments with clear proof.

The goal is simple: tuition support should go directly to tuition. Parents and sponsors can fund a specific invoice, students can confirm the invoice is correct, and schools can verify payment without relying on screenshots or manual bank reconciliation.

## Prerequisites

Install:

- Rust
- Stellar CLI
- Freighter Wallet set to Testnet
- Soroban SDK 26.0.0 or newer

Add the WASM target:

```bash
rustup target add wasm32v1-none
```

## Contract Functions

- `create_invoice` - school creates one tuition invoice for a student, amount, token, and school wallet.
- `confirm_invoice` - student confirms the invoice is correct before payment is accepted.
- `pay_invoice` - parent or sponsor pays the confirmed invoice directly to the school wallet.
- `get_invoice` - registrar, parent, or student checks invoice status.
- `balance_due` - returns the unpaid amount for the invoice.

## How to Build

```bash
stellar contract build
```

Alternative:

```bash
cargo build --target wasm32v1-none --release
```

## How to Test

```bash
cargo test
```

## How to Deploy to Testnet

Create and fund a testnet identity:

```bash
stellar keys generate --global my-key --network testnet
stellar keys fund my-key --network testnet
```

Deploy the contract:

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/tuition_sure.wasm \
  --source my-key \
  --network testnet
```

Copy the Contract ID from the output and verify it on Stellar Expert.

## Sample CLI Invocation

Create a tuition invoice:

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source school-key \
  --network testnet \
  -- \
  create_invoice \
  --invoice_id MIDTERM \
  --student <STUDENT_ADDRESS> \
  --school <SCHOOL_ADDRESS> \
  --token <USDC_TOKEN_CONTRACT_ADDRESS> \
  --amount 180000000000
```

Student confirms the invoice:

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source student-key \
  --network testnet \
  -- \
  confirm_invoice \
  --invoice_id MIDTERM
```

Parent pays the invoice:

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source parent-key \
  --network testnet \
  -- \
  pay_invoice \
  --invoice_id MIDTERM \
  --payer <PARENT_ADDRESS>
```

Check invoice status:

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source my-key \
  --network testnet \
  -- \
  get_invoice \
  --invoice_id MIDTERM
```

## License

MIT
