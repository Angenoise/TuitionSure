#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Invoice(Symbol),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Invoice {
    pub student: Address,
    pub school: Address,
    pub token: Address,
    pub amount: i128,
    pub confirmed: bool,
    pub paid: bool,
    pub payer: Option<Address>,
}

#[contract]
pub struct TuitionSure;

#[contractimpl]
impl TuitionSure {
    // School creates a tuition invoice for one student and one exact amount.
    pub fn create_invoice(
        env: Env,
        invoice_id: Symbol,
        student: Address,
        school: Address,
        token: Address,
        amount: i128,
    ) {
        school.require_auth();

        if amount <= 0 {
            panic!("amount must be positive");
        }

        let key = DataKey::Invoice(invoice_id);
        if env.storage().persistent().has(&key) {
            panic!("invoice already exists");
        }

        let invoice = Invoice {
            student,
            school,
            token,
            amount,
            confirmed: false,
            paid: false,
            payer: None,
        };

        env.storage().persistent().set(&key, &invoice);
    }

    // Student confirms the invoice is correct before any tuition payment is accepted.
    pub fn confirm_invoice(env: Env, invoice_id: Symbol) {
        let key = DataKey::Invoice(invoice_id);
        let mut invoice: Invoice = env
            .storage()
            .persistent()
            .get(&key)
            .expect("invoice not found");

        invoice.student.require_auth();

        if invoice.confirmed {
            panic!("invoice already confirmed");
        }

        invoice.confirmed = true;
        env.storage().persistent().set(&key, &invoice);
    }

    // Parent or sponsor pays the confirmed invoice directly to the school wallet.
    pub fn pay_invoice(env: Env, invoice_id: Symbol, payer: Address) {
        let key = DataKey::Invoice(invoice_id);
        let mut invoice: Invoice = env
            .storage()
            .persistent()
            .get(&key)
            .expect("invoice not found");

        if !invoice.confirmed {
            panic!("student has not confirmed invoice");
        }

        if invoice.paid {
            panic!("invoice already paid");
        }

        let token_client = token::Client::new(&env, &invoice.token);

        // Stellar token transfer moves funds from the parent/sponsor to the school.
        token_client.transfer(&payer, &invoice.school, &invoice.amount);

        invoice.paid = true;
        invoice.payer = Some(payer);
        env.storage().persistent().set(&key, &invoice);
    }

    // Anyone can check the invoice status for registrar, parent, or student proof.
    pub fn get_invoice(env: Env, invoice_id: Symbol) -> Invoice {
        let key = DataKey::Invoice(invoice_id);
        env.storage()
            .persistent()
            .get(&key)
            .expect("invoice not found")
    }

    // Returns the unpaid amount, useful for tuition dashboards and installment mode.
    pub fn balance_due(env: Env, invoice_id: Symbol) -> i128 {
        let invoice = Self::get_invoice(env, invoice_id);

        if invoice.paid {
            0
        } else {
            invoice.amount
        }
    }
}

#[cfg(test)]
mod test;