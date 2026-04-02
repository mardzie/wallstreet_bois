use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct BalanceSheet {
    active: Active,
    passive: Passive,
}

#[derive(Debug)]
pub struct Active {
    capital_assets: f64,
    current_assets: f64,
    prepaid_expenses: f64,
}

#[derive(Debug)]
pub struct Passive {
    equity: f64,
    liabilities: f64,
    differed_income: f64,
}

impl BalanceSheet {
    pub fn new(active: Active, passive: Passive) -> Self {
        assert_eq!(
            active.sum(),
            passive.sum(),
            "Active sum and Passive sum side of the BalanceSheet must be equal!"
        );

        Self { active, passive }
    }
}

impl Active {
    pub fn new(capital_assets: f64, current_assets: f64, prepaid_expenses: f64) -> Self {
        Self {
            capital_assets,
            current_assets,
            prepaid_expenses,
        }
    }

    pub fn sum(&self) -> f64 {
        self.capital_assets + self.current_assets + self.prepaid_expenses
    }
}

impl Passive {
    pub fn new(equity: f64, liabilities: f64, differed_income: f64) -> Self {
        Self {
            equity,
            liabilities,
            differed_income,
        }
    }

    pub fn sum(&self) -> f64 {
        self.differed_income + self.equity + self.liabilities
    }
}
