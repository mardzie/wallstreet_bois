use bevy::prelude::*;

use crate::companies::sector::Sector;

pub mod balance_sheet;
pub mod cash_flow_statement;
pub mod sector;

pub struct CompaniesPlugin;

impl Plugin for CompaniesPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component, Debug)]
pub struct Company {
    name: String,
    /// A description of the stocks company.
    about: String,
    sector: Sector,
}

impl Company {
    pub fn new(name: String, about: String, sector: Sector) -> Self {
        Self {
            name,
            about,
            sector,
        }
    }

    pub fn from_ref(name: &str, about: &str, sector: &Sector) -> Self {
        Self {
            name: name.to_string(),
            about: about.to_string(),
            sector: sector.clone(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn about(&self) -> &str {
        &self.about
    }

    pub fn sector(&self) -> &Sector {
        &self.sector
    }
}
