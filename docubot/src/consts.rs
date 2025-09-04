use lazy_static::lazy_static;

// BIT specific constantss
pub const BIT_TEMPERATURE: f32 = 0.34; // 0.34
pub const BIT_MAX_RESULTS: usize = 10; // 10
lazy_static! {
    pub static ref BIT_TEST_PAGE_NAMES: Vec<&'static str> = vec![
        "Salesforce is cloud-based",
        "CRM stands for Customer Relationship Management",
        "Salesforce automates workflows",
        "AppExchange is like an app store",
        "Einstein AI powers insights",
        "Trailhead teaches Salesforce"
    ];
}

// General constants
pub const BANNER: &str = r"
 ____   __    ___  _  _  ____   __  ____
(    \ /  \  / __)/ )( \(  _ \ /  \(_  _)
 ) D ((  O )( (__ ) \/ ( ) _ ((  O ) )(
(____/ \__/  \___)\____/(____/ \__/ (__)
          **<<Kilroy Was Here>>**
";
pub const TEMPERATURE: f32 = 0.34;
pub const MAX_RESULTS: usize = 10;
