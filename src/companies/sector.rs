use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sector {
    /// Oil, Gas, Renewables
    Energy,
    /// Mining, Chemicals, Packaging
    Materials,
    /// Aerospace, Transport, Machinery
    Industrials,
    /// Retail, Autos, Travel
    ConsumerDiscretionary,
    /// Food, Beverages, Household
    ConsumerStaples,
    /// Pharma, Biotech, Devices
    HealthCare,
    /// Banks, Insurances, Asset Management
    Financials,
    /// Software, Hardware, Chips
    Technology,
    /// Media, Telecom, Internet
    CommunicationServices,
    /// Electrical, Gas, Water
    Utilities,
    /// REITs, Property Companies
    RealEstate,
}

impl Display for Sector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Energy => write!(f, "Energy"),
            Self::Materials => write!(f, "Materials"),
            Self::Industrials => write!(f, "Industrials"),
            Self::ConsumerDiscretionary => write!(f, "Consumer Discretionary"),
            Self::ConsumerStaples => write!(f, "Consumer Staples"),
            Self::HealthCare => write!(f, "Health Care"),
            Self::Financials => write!(f, "Financials"),
            Self::Technology => write!(f, "Information Technology"),
            Self::CommunicationServices => write!(f, "Communication Services"),
            Self::Utilities => write!(f, "Utilities"),
            Self::RealEstate => write!(f, "Real Estate"),
        }
    }
}

impl Sector {
    pub fn description(&self) -> &str {
        match self {
            Self::Energy => "Oil, Gas, Renewables",
            Self::Materials => "Mining, Chemicals, Packaging",
            Self::Industrials => "Aerospace, Transport, Machinery",
            Self::ConsumerDiscretionary => "Retail, Autos, Travel",
            Self::ConsumerStaples => "Food, Beverages, Household",
            Self::HealthCare => "Pharma, Biotech, Devices",
            Self::Financials => "Banks, Insurances, Asset Management",
            Self::Technology => "Software, Hardware, Chips",
            Self::CommunicationServices => "Media, Telecom, Internet",
            Self::Utilities => "Electrical, Gas, Water",
            Self::RealEstate => "REITs, Property Companies",
        }
    }
}
