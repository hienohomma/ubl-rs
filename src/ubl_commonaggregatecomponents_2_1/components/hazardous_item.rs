use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HazardousItem {
    #[serde(rename = "AdditionalInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_information: Option<HazardousItemArrayOfAdditionalInformationComponent>,
    #[serde(rename = "AdditionalTemperature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_temperature: Option<HazardousItemArrayOfAdditionalTemperatureComponent>,
    #[serde(rename = "CategoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_name: Option<HazardousItemArrayOfCategoryNameComponent>,
    #[serde(rename = "ContactParty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_party: Option<HazardousItemArrayOfContactPartyComponent>,
    #[serde(rename = "EmergencyProceduresCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency_procedures_code: Option<HazardousItemArrayOfEmergencyProceduresCodeComponent>,
    #[serde(rename = "EmergencyTemperature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency_temperature: Option<HazardousItemArrayOfEmergencyTemperatureComponent>,
    #[serde(rename = "FlashpointTemperature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flashpoint_temperature: Option<HazardousItemArrayOfFlashpointTemperatureComponent>,
    #[serde(rename = "HazardClassID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hazard_class_id: Option<HazardousItemArrayOfHazardClassIDComponent>,
    #[serde(rename = "HazardousCategoryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hazardous_category_code: Option<HazardousItemArrayOfHazardousCategoryCodeComponent>,
    #[serde(rename = "HazardousGoodsTransit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hazardous_goods_transit: Option<HazardousItemArrayOfHazardousGoodsTransitComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<HazardousItemArrayOfIDComponent>,
    #[serde(rename = "LowerOrangeHazardPlacardID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_orange_hazard_placard_id: Option<HazardousItemArrayOfLowerOrangeHazardPlacardIDComponent>,
    #[serde(rename = "MarkingID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marking_id: Option<HazardousItemArrayOfMarkingIDComponent>,
    #[serde(rename = "MedicalFirstAidGuideCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_first_aid_guide_code: Option<HazardousItemArrayOfMedicalFirstAidGuideCodeComponent>,
    #[serde(rename = "NetVolumeMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_volume_measure: Option<HazardousItemArrayOfNetVolumeMeasureComponent>,
    #[serde(rename = "NetWeightMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_weight_measure: Option<HazardousItemArrayOfNetWeightMeasureComponent>,
    #[serde(rename = "PlacardEndorsement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placard_endorsement: Option<HazardousItemArrayOfPlacardEndorsementComponent>,
    #[serde(rename = "PlacardNotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placard_notation: Option<HazardousItemArrayOfPlacardNotationComponent>,
    #[serde(rename = "Quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<HazardousItemArrayOfQuantityComponent>,
    #[serde(rename = "SecondaryHazard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_hazard: Option<HazardousItemArrayOfSecondaryHazardComponent>,
    #[serde(rename = "TechnicalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_name: Option<HazardousItemArrayOfTechnicalNameComponent>,
    #[serde(rename = "UNDGCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undgcode: Option<HazardousItemArrayOfUNDGCodeComponent>,
    #[serde(rename = "UpperOrangeHazardPlacardID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_orange_hazard_placard_id: Option<HazardousItemArrayOfUpperOrangeHazardPlacardIDComponent>,
}

impl AsMut<HazardousItem> for HazardousItem {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItem> for HazardousItem {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.undgcode {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.undgcode", e));
            }
        }
        if let Some(v) = &self.category_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.category_name", e));
            }
        }
        if let Some(v) = &self.technical_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.technical_name", e));
            }
        }
        if let Some(v) = &self.emergency_procedures_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.emergency_procedures_code", e));
            }
        }
        if let Some(v) = &self.hazard_class_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.hazard_class_id", e));
            }
        }
        if let Some(v) = &self.medical_first_aid_guide_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.medical_first_aid_guide_code", e));
            }
        }
        if let Some(v) = &self.secondary_hazard {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.secondary_hazard", e));
            }
        }
        if let Some(v) = &self.placard_endorsement {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.placard_endorsement", e));
            }
        }
        if let Some(v) = &self.additional_information {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.additional_information", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.id", e));
            }
        }
        if let Some(v) = &self.hazardous_goods_transit {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.hazardous_goods_transit", e));
            }
        }
        if let Some(v) = &self.placard_notation {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.placard_notation", e));
            }
        }
        if let Some(v) = &self.quantity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.quantity", e));
            }
        }
        if let Some(v) = &self.flashpoint_temperature {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.flashpoint_temperature", e));
            }
        }
        if let Some(v) = &self.upper_orange_hazard_placard_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.upper_orange_hazard_placard_id", e));
            }
        }
        if let Some(v) = &self.additional_temperature {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.additional_temperature", e));
            }
        }
        if let Some(v) = &self.hazardous_category_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.hazardous_category_code", e));
            }
        }
        if let Some(v) = &self.lower_orange_hazard_placard_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.lower_orange_hazard_placard_id", e));
            }
        }
        if let Some(v) = &self.marking_id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.marking_id", e));
            }
        }
        if let Some(v) = &self.contact_party {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.contact_party", e));
            }
        }
        if let Some(v) = &self.net_weight_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.net_weight_measure", e));
            }
        }
        if let Some(v) = &self.emergency_temperature {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.emergency_temperature", e));
            }
        }
        if let Some(v) = &self.net_volume_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("HazardousItem.net_volume_measure", e));
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

impl HazardousItem {
    pub fn title() -> &'static str {
        "Hazardous Item. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a hazardous item."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            upper_orange_hazard_placard_id: None,
            contact_party: None,
            quantity: None,
            additional_information: None,
            category_name: None,
            technical_name: None,
            emergency_procedures_code: None,
            lower_orange_hazard_placard_id: None,
            net_weight_measure: None,
            id: None,
            secondary_hazard: None,
            medical_first_aid_guide_code: None,
            additional_temperature: None,
            flashpoint_temperature: None,
            hazardous_goods_transit: None,
            marking_id: None,
            net_volume_measure: None,
            hazardous_category_code: None,
            placard_notation: None,
            emergency_temperature: None,
            hazard_class_id: None,
            placard_endorsement: None,
            undgcode: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfAdditionalInformationComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AdditionalInformation>,
}

impl AsMut<HazardousItemArrayOfAdditionalInformationComponent> for HazardousItemArrayOfAdditionalInformationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfAdditionalInformationComponent> for HazardousItemArrayOfAdditionalInformationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfAdditionalInformationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfAdditionalInformationComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AdditionalInformation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AdditionalInformation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AdditionalInformation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AdditionalInformation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfAdditionalTemperatureComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AdditionalTemperature>,
}

impl AsMut<HazardousItemArrayOfAdditionalTemperatureComponent> for HazardousItemArrayOfAdditionalTemperatureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfAdditionalTemperatureComponent> for HazardousItemArrayOfAdditionalTemperatureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfAdditionalTemperatureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfAdditionalTemperatureComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AdditionalTemperature) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AdditionalTemperature) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AdditionalTemperature> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AdditionalTemperature> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfCategoryNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CategoryName>,
}

impl AsMut<HazardousItemArrayOfCategoryNameComponent> for HazardousItemArrayOfCategoryNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfCategoryNameComponent> for HazardousItemArrayOfCategoryNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfCategoryNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfCategoryNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfCategoryNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CategoryName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CategoryName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CategoryName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CategoryName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfContactPartyComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ContactParty>,
}

impl AsMut<HazardousItemArrayOfContactPartyComponent> for HazardousItemArrayOfContactPartyComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfContactPartyComponent> for HazardousItemArrayOfContactPartyComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfContactPartyComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfContactPartyComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfContactPartyComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ContactParty) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ContactParty) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ContactParty> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ContactParty> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfEmergencyProceduresCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::EmergencyProceduresCode>,
}

impl AsMut<HazardousItemArrayOfEmergencyProceduresCodeComponent> for HazardousItemArrayOfEmergencyProceduresCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfEmergencyProceduresCodeComponent> for HazardousItemArrayOfEmergencyProceduresCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfEmergencyProceduresCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfEmergencyProceduresCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfEmergencyProceduresCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::EmergencyProceduresCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::EmergencyProceduresCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::EmergencyProceduresCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::EmergencyProceduresCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfEmergencyTemperatureComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::EmergencyTemperature>,
}

impl AsMut<HazardousItemArrayOfEmergencyTemperatureComponent> for HazardousItemArrayOfEmergencyTemperatureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfEmergencyTemperatureComponent> for HazardousItemArrayOfEmergencyTemperatureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfEmergencyTemperatureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfEmergencyTemperatureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfEmergencyTemperatureComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::EmergencyTemperature) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::EmergencyTemperature) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::EmergencyTemperature> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::EmergencyTemperature> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfFlashpointTemperatureComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::FlashpointTemperature>,
}

impl AsMut<HazardousItemArrayOfFlashpointTemperatureComponent> for HazardousItemArrayOfFlashpointTemperatureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfFlashpointTemperatureComponent> for HazardousItemArrayOfFlashpointTemperatureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfFlashpointTemperatureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfFlashpointTemperatureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfFlashpointTemperatureComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::FlashpointTemperature) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::FlashpointTemperature) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::FlashpointTemperature> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::FlashpointTemperature> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfHazardClassIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HazardClassID>,
}

impl AsMut<HazardousItemArrayOfHazardClassIDComponent> for HazardousItemArrayOfHazardClassIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfHazardClassIDComponent> for HazardousItemArrayOfHazardClassIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfHazardClassIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfHazardClassIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfHazardClassIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::HazardClassID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::HazardClassID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::HazardClassID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::HazardClassID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfHazardousCategoryCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::HazardousCategoryCode>,
}

impl AsMut<HazardousItemArrayOfHazardousCategoryCodeComponent> for HazardousItemArrayOfHazardousCategoryCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfHazardousCategoryCodeComponent> for HazardousItemArrayOfHazardousCategoryCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfHazardousCategoryCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfHazardousCategoryCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfHazardousCategoryCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::HazardousCategoryCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::HazardousCategoryCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::HazardousCategoryCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::HazardousCategoryCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfHazardousGoodsTransitComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::HazardousGoodsTransit>,
}

impl AsMut<HazardousItemArrayOfHazardousGoodsTransitComponent> for HazardousItemArrayOfHazardousGoodsTransitComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfHazardousGoodsTransitComponent> for HazardousItemArrayOfHazardousGoodsTransitComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfHazardousGoodsTransitComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfHazardousGoodsTransitComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::HazardousGoodsTransit) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::HazardousGoodsTransit) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::HazardousGoodsTransit> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::HazardousGoodsTransit> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<HazardousItemArrayOfIDComponent> for HazardousItemArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfIDComponent> for HazardousItemArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::ID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::ID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::ID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::ID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfLowerOrangeHazardPlacardIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LowerOrangeHazardPlacardID>,
}

impl AsMut<HazardousItemArrayOfLowerOrangeHazardPlacardIDComponent> for HazardousItemArrayOfLowerOrangeHazardPlacardIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfLowerOrangeHazardPlacardIDComponent> for HazardousItemArrayOfLowerOrangeHazardPlacardIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfLowerOrangeHazardPlacardIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfLowerOrangeHazardPlacardIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfLowerOrangeHazardPlacardIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LowerOrangeHazardPlacardID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LowerOrangeHazardPlacardID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LowerOrangeHazardPlacardID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LowerOrangeHazardPlacardID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfMarkingIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MarkingID>,
}

impl AsMut<HazardousItemArrayOfMarkingIDComponent> for HazardousItemArrayOfMarkingIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfMarkingIDComponent> for HazardousItemArrayOfMarkingIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfMarkingIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfMarkingIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfMarkingIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MarkingID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MarkingID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MarkingID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MarkingID> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfMedicalFirstAidGuideCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MedicalFirstAidGuideCode>,
}

impl AsMut<HazardousItemArrayOfMedicalFirstAidGuideCodeComponent> for HazardousItemArrayOfMedicalFirstAidGuideCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfMedicalFirstAidGuideCodeComponent> for HazardousItemArrayOfMedicalFirstAidGuideCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfMedicalFirstAidGuideCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfMedicalFirstAidGuideCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfMedicalFirstAidGuideCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MedicalFirstAidGuideCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MedicalFirstAidGuideCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MedicalFirstAidGuideCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MedicalFirstAidGuideCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfNetVolumeMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NetVolumeMeasure>,
}

impl AsMut<HazardousItemArrayOfNetVolumeMeasureComponent> for HazardousItemArrayOfNetVolumeMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfNetVolumeMeasureComponent> for HazardousItemArrayOfNetVolumeMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfNetVolumeMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfNetVolumeMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfNetVolumeMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::NetVolumeMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::NetVolumeMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::NetVolumeMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::NetVolumeMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfNetWeightMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::NetWeightMeasure>,
}

impl AsMut<HazardousItemArrayOfNetWeightMeasureComponent> for HazardousItemArrayOfNetWeightMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfNetWeightMeasureComponent> for HazardousItemArrayOfNetWeightMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfNetWeightMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfNetWeightMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfNetWeightMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::NetWeightMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::NetWeightMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::NetWeightMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::NetWeightMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfPlacardEndorsementComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PlacardEndorsement>,
}

impl AsMut<HazardousItemArrayOfPlacardEndorsementComponent> for HazardousItemArrayOfPlacardEndorsementComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfPlacardEndorsementComponent> for HazardousItemArrayOfPlacardEndorsementComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfPlacardEndorsementComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfPlacardEndorsementComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfPlacardEndorsementComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PlacardEndorsement) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PlacardEndorsement) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PlacardEndorsement> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PlacardEndorsement> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfPlacardNotationComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PlacardNotation>,
}

impl AsMut<HazardousItemArrayOfPlacardNotationComponent> for HazardousItemArrayOfPlacardNotationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfPlacardNotationComponent> for HazardousItemArrayOfPlacardNotationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfPlacardNotationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfPlacardNotationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfPlacardNotationComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PlacardNotation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PlacardNotation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PlacardNotation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PlacardNotation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfQuantityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Quantity>,
}

impl AsMut<HazardousItemArrayOfQuantityComponent> for HazardousItemArrayOfQuantityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfQuantityComponent> for HazardousItemArrayOfQuantityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfQuantityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfQuantityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfQuantityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Quantity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Quantity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Quantity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Quantity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfSecondaryHazardComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SecondaryHazard>,
}

impl AsMut<HazardousItemArrayOfSecondaryHazardComponent> for HazardousItemArrayOfSecondaryHazardComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfSecondaryHazardComponent> for HazardousItemArrayOfSecondaryHazardComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfSecondaryHazardComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfSecondaryHazardComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SecondaryHazard) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SecondaryHazard) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SecondaryHazard> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SecondaryHazard> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfTechnicalNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TechnicalName>,
}

impl AsMut<HazardousItemArrayOfTechnicalNameComponent> for HazardousItemArrayOfTechnicalNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfTechnicalNameComponent> for HazardousItemArrayOfTechnicalNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfTechnicalNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfTechnicalNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfTechnicalNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TechnicalName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TechnicalName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TechnicalName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TechnicalName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfUNDGCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UNDGCode>,
}

impl AsMut<HazardousItemArrayOfUNDGCodeComponent> for HazardousItemArrayOfUNDGCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfUNDGCodeComponent> for HazardousItemArrayOfUNDGCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfUNDGCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfUNDGCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfUNDGCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::UNDGCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::UNDGCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::UNDGCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::UNDGCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HazardousItemArrayOfUpperOrangeHazardPlacardIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::UpperOrangeHazardPlacardID>,
}

impl AsMut<HazardousItemArrayOfUpperOrangeHazardPlacardIDComponent> for HazardousItemArrayOfUpperOrangeHazardPlacardIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<HazardousItemArrayOfUpperOrangeHazardPlacardIDComponent> for HazardousItemArrayOfUpperOrangeHazardPlacardIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfUpperOrangeHazardPlacardIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("HazardousItemArrayOfUpperOrangeHazardPlacardIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl HazardousItemArrayOfUpperOrangeHazardPlacardIDComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::UpperOrangeHazardPlacardID) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::UpperOrangeHazardPlacardID) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::UpperOrangeHazardPlacardID> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::UpperOrangeHazardPlacardID> {
        self.items.iter()
    }
}

