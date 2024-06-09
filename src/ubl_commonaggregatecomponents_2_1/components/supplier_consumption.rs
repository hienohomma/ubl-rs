use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SupplierConsumption {
    #[serde(rename = "Consumption")]
    pub consumption: SupplierConsumptionArrayOfConsumptionComponent,
    #[serde(rename = "ConsumptionLine")]
    pub consumption_line: SupplierConsumptionArrayOfConsumptionLineComponent,
    #[serde(rename = "Contract")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<SupplierConsumptionArrayOfContractComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<SupplierConsumptionArrayOfDescriptionComponent>,
    #[serde(rename = "UtilityCustomerParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utility_customer_party: Option<SupplierConsumptionArrayOfUtilityCustomerPartyComponent>,
    #[serde(rename = "UtilitySupplierParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utility_supplier_party: Option<SupplierConsumptionArrayOfUtilitySupplierPartyComponent>,
}

impl AsMut<SupplierConsumption> for SupplierConsumption {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SupplierConsumption> for SupplierConsumption {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.consumption_line.validate() {
            return Err(UblError::component("SupplierConsumption.consumption_line", e));
        }
        if let Err(e) = self.consumption.validate() {
            return Err(UblError::component("SupplierConsumption.consumption", e));
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SupplierConsumption.description", e));
            }
        }
        if let Some(v) = &self.contract {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SupplierConsumption.contract", e));
            }
        }
        if let Some(v) = &self.utility_customer_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SupplierConsumption.utility_customer_party", e));
            }
        }
        if let Some(v) = &self.utility_supplier_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SupplierConsumption.utility_supplier_party", e));
            }
        }

        Ok(self)
    }

    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl SupplierConsumption {
    pub fn title() -> &'static str {
        "Supplier Consumption. Details"
    }
    pub fn description() -> &'static str {
        "The consumption in case the consumption is for one and only one supplier."
    }
    pub fn new(consumption: SupplierConsumptionArrayOfConsumptionComponent, consumption_line: SupplierConsumptionArrayOfConsumptionLineComponent) -> Component<Self> {
        Component(Self {
            contract: None,
            description: None,
            consumption_line,
            consumption,
            utility_supplier_party: None,
            utility_customer_party: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SupplierConsumptionArrayOfConsumptionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Consumption>,
}

impl AsMut<SupplierConsumptionArrayOfConsumptionComponent> for SupplierConsumptionArrayOfConsumptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SupplierConsumptionArrayOfConsumptionComponent> for SupplierConsumptionArrayOfConsumptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SupplierConsumptionArrayOfConsumptionComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SupplierConsumptionArrayOfConsumptionComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl SupplierConsumptionArrayOfConsumptionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Consumption) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Consumption) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Consumption> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Consumption> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SupplierConsumptionArrayOfConsumptionLineComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ConsumptionLine>,
}

impl AsMut<SupplierConsumptionArrayOfConsumptionLineComponent> for SupplierConsumptionArrayOfConsumptionLineComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SupplierConsumptionArrayOfConsumptionLineComponent> for SupplierConsumptionArrayOfConsumptionLineComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("SupplierConsumptionArrayOfConsumptionLineComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl SupplierConsumptionArrayOfConsumptionLineComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ConsumptionLine) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ConsumptionLine) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ConsumptionLine> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ConsumptionLine> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SupplierConsumptionArrayOfContractComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Contract>,
}

impl AsMut<SupplierConsumptionArrayOfContractComponent> for SupplierConsumptionArrayOfContractComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SupplierConsumptionArrayOfContractComponent> for SupplierConsumptionArrayOfContractComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SupplierConsumptionArrayOfContractComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SupplierConsumptionArrayOfContractComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl SupplierConsumptionArrayOfContractComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Contract) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Contract) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Contract> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Contract> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SupplierConsumptionArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<SupplierConsumptionArrayOfDescriptionComponent> for SupplierConsumptionArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SupplierConsumptionArrayOfDescriptionComponent> for SupplierConsumptionArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("SupplierConsumptionArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl SupplierConsumptionArrayOfDescriptionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Description) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Description) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Description> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Description> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SupplierConsumptionArrayOfUtilityCustomerPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::UtilityCustomerParty>,
}

impl AsMut<SupplierConsumptionArrayOfUtilityCustomerPartyComponent> for SupplierConsumptionArrayOfUtilityCustomerPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SupplierConsumptionArrayOfUtilityCustomerPartyComponent> for SupplierConsumptionArrayOfUtilityCustomerPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SupplierConsumptionArrayOfUtilityCustomerPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SupplierConsumptionArrayOfUtilityCustomerPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl SupplierConsumptionArrayOfUtilityCustomerPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::UtilityCustomerParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::UtilityCustomerParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::UtilityCustomerParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::UtilityCustomerParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SupplierConsumptionArrayOfUtilitySupplierPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::UtilitySupplierParty>,
}

impl AsMut<SupplierConsumptionArrayOfUtilitySupplierPartyComponent> for SupplierConsumptionArrayOfUtilitySupplierPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SupplierConsumptionArrayOfUtilitySupplierPartyComponent> for SupplierConsumptionArrayOfUtilitySupplierPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SupplierConsumptionArrayOfUtilitySupplierPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SupplierConsumptionArrayOfUtilitySupplierPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
        }

        Ok(self)
    }
    fn get(self) -> Result<Self, UblError> {
        self.validate().map(|s|s.clone())
    }
    fn additional_props_allowed() -> bool {
        false
    }
}

impl SupplierConsumptionArrayOfUtilitySupplierPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::UtilitySupplierParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::UtilitySupplierParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::UtilitySupplierParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::UtilitySupplierParty> {
        self.items.iter()
    }
}

