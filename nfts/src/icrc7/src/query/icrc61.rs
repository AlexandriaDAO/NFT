use crate::types::Standard;

#[ic_cdk::query]
pub fn icrc61_supported_standards() -> Vec<Standard> {
    vec![
        Standard {
            name: "ICRC-7".into(),
            url: "https://github.com/dfinity/ICRC/tree/main/ICRCs/ICRC-7".into(),
        },
        Standard {
            name: "ICRC-37".into(),
            url: "https://github.com/dfinity/ICRC/tree/main/ICRCs/ICRC-37".into(),
        },
        Standard {
            name: "ICRC-61".into(),
            url: "https://github.com/dfinity/ICRC/tree/main/ICRCs/ICRC-61".into(),
        },
    ]
}
