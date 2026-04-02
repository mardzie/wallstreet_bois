use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct CashFlowStatement {
    operating: f64,
    capital_expendeture: f64,
    free: f64,
    investing: f64,
    financing: f64,
    end: f64,
    issurance_of_debt: f64,
    repayment_of_debt: f64,
}

impl CashFlowStatement {
    pub fn new(
        operating: f64,
        capital_expendeture: f64,
        investing: f64,
        end: f64,
        issurance_of_debt: f64,
        repayment_of_debt: f64,
    ) -> Self {
        Self {
            operating,
            capital_expendeture,
            free: operating - capital_expendeture,
            investing,
            financing: issurance_of_debt - repayment_of_debt,
            end,
            issurance_of_debt,
            repayment_of_debt,
        }
    }
}
