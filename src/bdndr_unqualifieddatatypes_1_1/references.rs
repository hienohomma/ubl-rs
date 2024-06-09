#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct CodeType (pub crate::bdndr_ccts_cct_schemamodule_1_1::CodeType);

impl CodeType {
    pub fn title() -> &'static str {
        "Code. Type"
    }
    pub fn description() -> &'static str {
        "A character string (letters, figures, or symbols) that for brevity and/or language independence may be used to represent or replace a definitive value or text of an attribute, together with relevant supplementary information."
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct IdentifierType (pub crate::bdndr_ccts_cct_schemamodule_1_1::IdentifierType);

impl IdentifierType {
    pub fn title() -> &'static str {
        "Identifier. Type"
    }
    pub fn description() -> &'static str {
        "A character string to identify and uniquely distinguish one instance of an object in an identification scheme from all other objects in the same scheme, together with relevant supplementary information."
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct NameType (pub crate::bdndr_ccts_cct_schemamodule_1_1::TextType);

impl NameType {
    pub fn title() -> &'static str {
        "Name. Type"
    }
    pub fn description() -> &'static str {
        "A character string that constitutes the distinctive designation of a person, place, thing or concept."
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct NumericType (pub crate::bdndr_ccts_cct_schemamodule_1_1::NumericType);

impl NumericType {
    pub fn title() -> &'static str {
        "Numeric. Type"
    }
    pub fn description() -> &'static str {
        "Numeric information that is assigned or is determined by calculation, counting, or sequencing. It does not require a unit of quantity or unit of measure."
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct PercentType (pub crate::bdndr_ccts_cct_schemamodule_1_1::NumericType);

impl PercentType {
    pub fn title() -> &'static str {
        "Percent. Type"
    }
    pub fn description() -> &'static str {
        "Numeric information that is assigned or is determined by calculation, counting, or sequencing and is expressed as a percentage. It does not require a unit of quantity or unit of measure."
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct QuantityType (pub crate::bdndr_ccts_cct_schemamodule_1_1::QuantityType);

impl QuantityType {
    pub fn title() -> &'static str {
        "Quantity. Type"
    }
    pub fn description() -> &'static str {
        "A counted number of non-monetary units, possibly including a fractional part."
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct RateType (pub crate::bdndr_ccts_cct_schemamodule_1_1::NumericType);

impl RateType {
    pub fn title() -> &'static str {
        "Rate. Type"
    }
    pub fn description() -> &'static str {
        "A numeric expression of a rate that is assigned or is determined by calculation, counting, or sequencing. It does not require a unit of quantity or unit of measure."
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TextType (pub crate::bdndr_ccts_cct_schemamodule_1_1::TextType);

impl TextType {
    pub fn title() -> &'static str {
        "Text. Type"
    }
    pub fn description() -> &'static str {
        "A character string (i.e. a finite set of characters), generally in the form of words of a language."
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ValueType (pub crate::bdndr_ccts_cct_schemamodule_1_1::NumericType);

impl ValueType {
    pub fn title() -> &'static str {
        "Value. Type"
    }
    pub fn description() -> &'static str {
        "Numeric information that is assigned or is determined by calculation, counting, or sequencing. It does not require a unit of quantity or unit of measure."
    }
}
