#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum EsVersion {
    //2016
    ES7,
    //2017
    ES8,
    //2018
    ES9,
    //2019
    ES10,
    //2020
    ES11,
    //2021
    ES12,
    //2022
    ES13,
    //2023
    ES14,
    //2024
    ES15,
    //2025
    ES16,
    ESNext, // Future version placeholder
}

impl EsVersion {
    pub fn current_version(&self) -> &'static str {
        match self {
            EsVersion::ES7 => "ES7",
            EsVersion::ES8 => "ES8",
            EsVersion::ES9 => "ES9",
            EsVersion::ES10 => "ES10",
            EsVersion::ES11 => "ES11",
            EsVersion::ES12 => "ES12",
            EsVersion::ES13 => "ES13",
            EsVersion::ES14 => "ES14",
            EsVersion::ES15 => "ES15",
            EsVersion::ES16 => "ES16",
            EsVersion::ESNext => "ESNext",
        }
    }
}