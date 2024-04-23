use strum_macros::Display;

#[derive(Display, Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum EsVersion {
    ES7,
    ES8,
    ES9,
    ES10,
    ES11,
    ES12,
    ES13,
    ES14,
    ES15,
    ES16,
    ESNext,
}
