use crate::companies::sector::Sector;

pub const ALL_STOCKS: &[(&str, &str, &str, Sector, u64)] = &[
    (
        "Tesla Inc",
        "TSLA",
        "Tesla, Inc. designs, develops, manufactures, leases, and sells electric vehicles, and energy generation and storage systems in the United States, China, and internationally.",
        Sector::ConsumerDiscretionary,
        36000,
    ),
    (
        "NVIDIA Corporation",
        "NVDA",
        "NVIDIA Corporation operates as a data center scale AI infrastructure company.",
        Sector::Technology,
        16700,
    ),
    (
        "Intel Corporation",
        "INTC",
        "Intel Corporation designs, develops, manufactures, markets, sells, and services computing and related end products and services in the United States, Ireland, Israel, and internationally.",
        Sector::Technology,
        4300,
    ),
    (
        "Apple Inc.",
        "AAPL",
        "Apple Inc. designs, manufactures, and markets smartphones, personal computers, tablets, wearables, and accessories worldwide.",
        Sector::Technology,
        24880,
    ),
];
