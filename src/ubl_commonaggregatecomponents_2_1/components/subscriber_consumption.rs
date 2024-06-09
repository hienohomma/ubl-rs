use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubscriberConsumption {
    #[serde(rename = "Consumption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption: Option<SubscriberConsumptionArrayOfConsumptionComponent>,
    #[serde(rename = "ConsumptionID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_id: Option<SubscriberConsumptionArrayOfConsumptionIDComponent>,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<SubscriberConsumptionArrayOfNoteComponent>,
    #[serde(rename = "OnAccountPayment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_account_payment: Option<SubscriberConsumptionArrayOfOnAccountPaymentComponent>,
    #[serde(rename = "SpecificationTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specification_type_code: Option<SubscriberConsumptionArrayOfSpecificationTypeCodeComponent>,
    #[serde(rename = "SubscriberParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_party: Option<SubscriberConsumptionArrayOfSubscriberPartyComponent>,
    #[serde(rename = "SupplierConsumption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_consumption: Option<SubscriberConsumptionArrayOfSupplierConsumptionComponent>,
    #[serde(rename = "TotalMeteredQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_metered_quantity: Option<SubscriberConsumptionArrayOfTotalMeteredQuantityComponent>,
    #[serde(rename = "UtilityConsumptionPoint")]
    pub utility_consumption_point: SubscriberConsumptionArrayOfUtilityConsumptionPointComponent,
}

impl AsMut<SubscriberConsumption> for SubscriberConsumption {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SubscriberConsumption> for SubscriberConsumption {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.subscriber_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SubscriberConsumption.subscriber_party", e));
            }
        }
        if let Some(v) = &self.on_account_payment {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SubscriberConsumption.on_account_payment", e));
            }
        }
        if let Err(e) = self.utility_consumption_point.validate() {
            return Err(UblError::component("SubscriberConsumption.utility_consumption_point", e));
        }
        if let Some(v) = &self.consumption {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SubscriberConsumption.consumption", e));
            }
        }
        if let Some(v) = &self.note {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SubscriberConsumption.note", e));
            }
        }
        if let Some(v) = &self.specification_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SubscriberConsumption.specification_type_code", e));
            }
        }
        if let Some(v) = &self.consumption_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SubscriberConsumption.consumption_id", e));
            }
        }
        if let Some(v) = &self.supplier_consumption {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SubscriberConsumption.supplier_consumption", e));
            }
        }
        if let Some(v) = &self.total_metered_quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("SubscriberConsumption.total_metered_quantity", e));
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

impl SubscriberConsumption {
    pub fn title() -> &'static str {
        "Subscriber Consumption. Details"
    }
    pub fn description() -> &'static str {
        "The consumption for a specific party for given consumption point provided by a numbers of suppliers. An enterprise can have one utility statement for several parties (e.g. a ministry of defence receiving a telephone bill). In this way each subscriber consumption represent a sub utility statement."
    }
    pub fn new(utility_consumption_point: SubscriberConsumptionArrayOfUtilityConsumptionPointComponent) -> Component<Self> {
        Component(Self {
            consumption: None,
            consumption_id: None,
            utility_consumption_point,
            supplier_consumption: None,
            note: None,
            specification_type_code: None,
            on_account_payment: None,
            subscriber_party: None,
            total_metered_quantity: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SubscriberConsumptionArrayOfConsumptionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Consumption>,
}

impl AsMut<SubscriberConsumptionArrayOfConsumptionComponent> for SubscriberConsumptionArrayOfConsumptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SubscriberConsumptionArrayOfConsumptionComponent> for SubscriberConsumptionArrayOfConsumptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SubscriberConsumptionArrayOfConsumptionComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SubscriberConsumptionArrayOfConsumptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SubscriberConsumptionArrayOfConsumptionComponent {
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
pub struct SubscriberConsumptionArrayOfConsumptionIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ConsumptionID>,
}

impl AsMut<SubscriberConsumptionArrayOfConsumptionIDComponent> for SubscriberConsumptionArrayOfConsumptionIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SubscriberConsumptionArrayOfConsumptionIDComponent> for SubscriberConsumptionArrayOfConsumptionIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SubscriberConsumptionArrayOfConsumptionIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SubscriberConsumptionArrayOfConsumptionIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SubscriberConsumptionArrayOfConsumptionIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ConsumptionID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ConsumptionID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ConsumptionID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ConsumptionID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SubscriberConsumptionArrayOfNoteComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Note>,
}

impl AsMut<SubscriberConsumptionArrayOfNoteComponent> for SubscriberConsumptionArrayOfNoteComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SubscriberConsumptionArrayOfNoteComponent> for SubscriberConsumptionArrayOfNoteComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("SubscriberConsumptionArrayOfNoteComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SubscriberConsumptionArrayOfNoteComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Note) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Note) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Note> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Note> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SubscriberConsumptionArrayOfOnAccountPaymentComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::OnAccountPayment>,
}

impl AsMut<SubscriberConsumptionArrayOfOnAccountPaymentComponent> for SubscriberConsumptionArrayOfOnAccountPaymentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SubscriberConsumptionArrayOfOnAccountPaymentComponent> for SubscriberConsumptionArrayOfOnAccountPaymentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("SubscriberConsumptionArrayOfOnAccountPaymentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SubscriberConsumptionArrayOfOnAccountPaymentComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::OnAccountPayment) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::OnAccountPayment) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::OnAccountPayment> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::OnAccountPayment> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SubscriberConsumptionArrayOfSpecificationTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::SpecificationTypeCode>,
}

impl AsMut<SubscriberConsumptionArrayOfSpecificationTypeCodeComponent> for SubscriberConsumptionArrayOfSpecificationTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SubscriberConsumptionArrayOfSpecificationTypeCodeComponent> for SubscriberConsumptionArrayOfSpecificationTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SubscriberConsumptionArrayOfSpecificationTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SubscriberConsumptionArrayOfSpecificationTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SubscriberConsumptionArrayOfSpecificationTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::SpecificationTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::SpecificationTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::SpecificationTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::SpecificationTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SubscriberConsumptionArrayOfSubscriberPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SubscriberParty>,
}

impl AsMut<SubscriberConsumptionArrayOfSubscriberPartyComponent> for SubscriberConsumptionArrayOfSubscriberPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SubscriberConsumptionArrayOfSubscriberPartyComponent> for SubscriberConsumptionArrayOfSubscriberPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SubscriberConsumptionArrayOfSubscriberPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SubscriberConsumptionArrayOfSubscriberPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SubscriberConsumptionArrayOfSubscriberPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SubscriberParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SubscriberParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SubscriberParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SubscriberParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SubscriberConsumptionArrayOfSupplierConsumptionComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SupplierConsumption>,
}

impl AsMut<SubscriberConsumptionArrayOfSupplierConsumptionComponent> for SubscriberConsumptionArrayOfSupplierConsumptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SubscriberConsumptionArrayOfSupplierConsumptionComponent> for SubscriberConsumptionArrayOfSupplierConsumptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("SubscriberConsumptionArrayOfSupplierConsumptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SubscriberConsumptionArrayOfSupplierConsumptionComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SupplierConsumption) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SupplierConsumption) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SupplierConsumption> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SupplierConsumption> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SubscriberConsumptionArrayOfTotalMeteredQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TotalMeteredQuantity>,
}

impl AsMut<SubscriberConsumptionArrayOfTotalMeteredQuantityComponent> for SubscriberConsumptionArrayOfTotalMeteredQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SubscriberConsumptionArrayOfTotalMeteredQuantityComponent> for SubscriberConsumptionArrayOfTotalMeteredQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SubscriberConsumptionArrayOfTotalMeteredQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SubscriberConsumptionArrayOfTotalMeteredQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SubscriberConsumptionArrayOfTotalMeteredQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TotalMeteredQuantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TotalMeteredQuantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TotalMeteredQuantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TotalMeteredQuantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SubscriberConsumptionArrayOfUtilityConsumptionPointComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::UtilityConsumptionPoint>,
}

impl AsMut<SubscriberConsumptionArrayOfUtilityConsumptionPointComponent> for SubscriberConsumptionArrayOfUtilityConsumptionPointComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<SubscriberConsumptionArrayOfUtilityConsumptionPointComponent> for SubscriberConsumptionArrayOfUtilityConsumptionPointComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("SubscriberConsumptionArrayOfUtilityConsumptionPointComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("SubscriberConsumptionArrayOfUtilityConsumptionPointComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl SubscriberConsumptionArrayOfUtilityConsumptionPointComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::UtilityConsumptionPoint) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::UtilityConsumptionPoint) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::UtilityConsumptionPoint> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::UtilityConsumptionPoint> {
        self.items.iter()
    }
}

