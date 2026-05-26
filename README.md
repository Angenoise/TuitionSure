# TuitionSure

TuitionSure makes sure tuition money reaches the verified school wallet, not a student's personal spending wallet.

## Problem

An OFW parent in Dubai sends PHP 18,000 tuition money to their college student in Laguna, but the money can be delayed, misused, or mixed with allowance, causing the student to miss enrollment validation or exam permit release.

## Solution

TuitionSure lets the school create a tuition invoice, lets the parent pay that invoice through Stellar USDC, and uses a Soroban smart contract to make sure the payment can only be recorded and released to the verified school wallet.

## Timeline

- Bootcamp ideation and contract design
- Soroban smart contract implementation
- Local unit testing with 5 tests
- Stellar testnet deployment
- Rise In submission with GitHub repository and Contract ID

## Stellar Features Used

- USDC transfers
- Soroban smart contracts
- Trustlines

## Vision and Purpose

TuitionSure helps Filipino students, parents, OFW guardians, scholarship sponsors, and schools coordinate tuition payments with clear proof of payment.

The purpose is simple: tuition support should go directly to tuition. Parents and sponsors can fund a specific invoice, students can confirm the invoice is correct, and schools can verify payment status without relying on screenshots or manual bank reconciliation.

## Prerequisites

Install:

- Rust
- Stellar CLI
- Freighter Wallet set to Testnet
- Soroban SDK 26.0.0

Add the WASM target:

```bash
rustup target add wasm32v1-none



