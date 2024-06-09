use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Address {
    #[serde(rename = "AdditionalStreetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_street_name: Option<AddressArrayOfAdditionalStreetNameComponent>,
    #[serde(rename = "AddressFormatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_format_code: Option<AddressArrayOfAddressFormatCodeComponent>,
    #[serde(rename = "AddressLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line: Option<AddressArrayOfAddressLineComponent>,
    #[serde(rename = "AddressTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type_code: Option<AddressArrayOfAddressTypeCodeComponent>,
    #[serde(rename = "BlockName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_name: Option<AddressArrayOfBlockNameComponent>,
    #[serde(rename = "BuildingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building_name: Option<AddressArrayOfBuildingNameComponent>,
    #[serde(rename = "BuildingNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building_number: Option<AddressArrayOfBuildingNumberComponent>,
    #[serde(rename = "CityName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_name: Option<AddressArrayOfCityNameComponent>,
    #[serde(rename = "CitySubdivisionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_subdivision_name: Option<AddressArrayOfCitySubdivisionNameComponent>,
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<AddressArrayOfCountryComponent>,
    #[serde(rename = "CountrySubentity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_subentity: Option<AddressArrayOfCountrySubentityComponent>,
    #[serde(rename = "CountrySubentityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_subentity_code: Option<AddressArrayOfCountrySubentityCodeComponent>,
    #[serde(rename = "Department")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<AddressArrayOfDepartmentComponent>,
    #[serde(rename = "District")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<AddressArrayOfDistrictComponent>,
    #[serde(rename = "Floor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<AddressArrayOfFloorComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<AddressArrayOfIDComponent>,
    #[serde(rename = "InhouseMail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inhouse_mail: Option<AddressArrayOfInhouseMailComponent>,
    #[serde(rename = "LocationCoordinate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_coordinate: Option<AddressArrayOfLocationCoordinateComponent>,
    #[serde(rename = "MarkAttention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_attention: Option<AddressArrayOfMarkAttentionComponent>,
    #[serde(rename = "MarkCare")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_care: Option<AddressArrayOfMarkCareComponent>,
    #[serde(rename = "PlotIdentification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plot_identification: Option<AddressArrayOfPlotIdentificationComponent>,
    #[serde(rename = "PostalZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_zone: Option<AddressArrayOfPostalZoneComponent>,
    #[serde(rename = "Postbox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postbox: Option<AddressArrayOfPostboxComponent>,
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<AddressArrayOfRegionComponent>,
    #[serde(rename = "Room")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room: Option<AddressArrayOfRoomComponent>,
    #[serde(rename = "StreetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_name: Option<AddressArrayOfStreetNameComponent>,
    #[serde(rename = "TimezoneOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone_offset: Option<AddressArrayOfTimezoneOffsetComponent>,
}

impl AsMut<Address> for Address {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Address> for Address {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.country_subentity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.country_subentity", e));
            }
        }
        if let Some(v) = &self.country {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.country", e));
            }
        }
        if let Some(v) = &self.postal_zone {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.postal_zone", e));
            }
        }
        if let Some(v) = &self.additional_street_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.additional_street_name", e));
            }
        }
        if let Some(v) = &self.building_number {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.building_number", e));
            }
        }
        if let Some(v) = &self.city_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.city_name", e));
            }
        }
        if let Some(v) = &self.department {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.department", e));
            }
        }
        if let Some(v) = &self.address_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.address_type_code", e));
            }
        }
        if let Some(v) = &self.floor {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.floor", e));
            }
        }
        if let Some(v) = &self.location_coordinate {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.location_coordinate", e));
            }
        }
        if let Some(v) = &self.postbox {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.postbox", e));
            }
        }
        if let Some(v) = &self.region {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.region", e));
            }
        }
        if let Some(v) = &self.room {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.room", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.id", e));
            }
        }
        if let Some(v) = &self.city_subdivision_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.city_subdivision_name", e));
            }
        }
        if let Some(v) = &self.building_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.building_name", e));
            }
        }
        if let Some(v) = &self.mark_attention {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.mark_attention", e));
            }
        }
        if let Some(v) = &self.plot_identification {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.plot_identification", e));
            }
        }
        if let Some(v) = &self.inhouse_mail {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.inhouse_mail", e));
            }
        }
        if let Some(v) = &self.street_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.street_name", e));
            }
        }
        if let Some(v) = &self.country_subentity_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.country_subentity_code", e));
            }
        }
        if let Some(v) = &self.timezone_offset {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.timezone_offset", e));
            }
        }
        if let Some(v) = &self.address_line {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.address_line", e));
            }
        }
        if let Some(v) = &self.block_name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.block_name", e));
            }
        }
        if let Some(v) = &self.district {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.district", e));
            }
        }
        if let Some(v) = &self.address_format_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.address_format_code", e));
            }
        }
        if let Some(v) = &self.mark_care {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Address.mark_care", e));
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

impl Address {
    pub fn title() -> &'static str {
        "Address. Details"
    }
    pub fn description() -> &'static str {
        "A class to define common information related to an address."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            id: None,
            floor: None,
            additional_street_name: None,
            postal_zone: None,
            district: None,
            address_format_code: None,
            country: None,
            mark_attention: None,
            postbox: None,
            address_type_code: None,
            city_subdivision_name: None,
            department: None,
            location_coordinate: None,
            room: None,
            street_name: None,
            timezone_offset: None,
            inhouse_mail: None,
            plot_identification: None,
            country_subentity_code: None,
            building_number: None,
            city_name: None,
            mark_care: None,
            block_name: None,
            building_name: None,
            region: None,
            country_subentity: None,
            address_line: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfAdditionalStreetNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AdditionalStreetName>,
}

impl AsMut<AddressArrayOfAdditionalStreetNameComponent> for AddressArrayOfAdditionalStreetNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfAdditionalStreetNameComponent> for AddressArrayOfAdditionalStreetNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfAdditionalStreetNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfAdditionalStreetNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfAdditionalStreetNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AdditionalStreetName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AdditionalStreetName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AdditionalStreetName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AdditionalStreetName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfAddressFormatCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AddressFormatCode>,
}

impl AsMut<AddressArrayOfAddressFormatCodeComponent> for AddressArrayOfAddressFormatCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfAddressFormatCodeComponent> for AddressArrayOfAddressFormatCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfAddressFormatCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfAddressFormatCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfAddressFormatCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AddressFormatCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AddressFormatCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AddressFormatCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AddressFormatCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfAddressLineComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::AddressLine>,
}

impl AsMut<AddressArrayOfAddressLineComponent> for AddressArrayOfAddressLineComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfAddressLineComponent> for AddressArrayOfAddressLineComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfAddressLineComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfAddressLineComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::AddressLine) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::AddressLine) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::AddressLine> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::AddressLine> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfAddressTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AddressTypeCode>,
}

impl AsMut<AddressArrayOfAddressTypeCodeComponent> for AddressArrayOfAddressTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfAddressTypeCodeComponent> for AddressArrayOfAddressTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfAddressTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfAddressTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfAddressTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AddressTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AddressTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AddressTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AddressTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfBlockNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BlockName>,
}

impl AsMut<AddressArrayOfBlockNameComponent> for AddressArrayOfBlockNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfBlockNameComponent> for AddressArrayOfBlockNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfBlockNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfBlockNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfBlockNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BlockName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BlockName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BlockName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BlockName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfBuildingNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BuildingName>,
}

impl AsMut<AddressArrayOfBuildingNameComponent> for AddressArrayOfBuildingNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfBuildingNameComponent> for AddressArrayOfBuildingNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfBuildingNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfBuildingNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfBuildingNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BuildingName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BuildingName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BuildingName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BuildingName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfBuildingNumberComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::BuildingNumber>,
}

impl AsMut<AddressArrayOfBuildingNumberComponent> for AddressArrayOfBuildingNumberComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfBuildingNumberComponent> for AddressArrayOfBuildingNumberComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfBuildingNumberComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfBuildingNumberComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfBuildingNumberComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::BuildingNumber) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::BuildingNumber) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::BuildingNumber> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::BuildingNumber> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfCityNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CityName>,
}

impl AsMut<AddressArrayOfCityNameComponent> for AddressArrayOfCityNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfCityNameComponent> for AddressArrayOfCityNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfCityNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfCityNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfCityNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CityName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CityName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CityName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CityName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfCitySubdivisionNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CitySubdivisionName>,
}

impl AsMut<AddressArrayOfCitySubdivisionNameComponent> for AddressArrayOfCitySubdivisionNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfCitySubdivisionNameComponent> for AddressArrayOfCitySubdivisionNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfCitySubdivisionNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfCitySubdivisionNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfCitySubdivisionNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CitySubdivisionName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CitySubdivisionName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CitySubdivisionName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CitySubdivisionName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfCountryComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Country>,
}

impl AsMut<AddressArrayOfCountryComponent> for AddressArrayOfCountryComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfCountryComponent> for AddressArrayOfCountryComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfCountryComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfCountryComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfCountryComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Country) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Country) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Country> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Country> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfCountrySubentityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CountrySubentity>,
}

impl AsMut<AddressArrayOfCountrySubentityComponent> for AddressArrayOfCountrySubentityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfCountrySubentityComponent> for AddressArrayOfCountrySubentityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfCountrySubentityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfCountrySubentityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfCountrySubentityComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CountrySubentity) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CountrySubentity) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CountrySubentity> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CountrySubentity> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfCountrySubentityCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CountrySubentityCode>,
}

impl AsMut<AddressArrayOfCountrySubentityCodeComponent> for AddressArrayOfCountrySubentityCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfCountrySubentityCodeComponent> for AddressArrayOfCountrySubentityCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfCountrySubentityCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfCountrySubentityCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfCountrySubentityCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CountrySubentityCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CountrySubentityCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CountrySubentityCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CountrySubentityCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfDepartmentComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Department>,
}

impl AsMut<AddressArrayOfDepartmentComponent> for AddressArrayOfDepartmentComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfDepartmentComponent> for AddressArrayOfDepartmentComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfDepartmentComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfDepartmentComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfDepartmentComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Department) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Department) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Department> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Department> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfDistrictComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::District>,
}

impl AsMut<AddressArrayOfDistrictComponent> for AddressArrayOfDistrictComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfDistrictComponent> for AddressArrayOfDistrictComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfDistrictComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfDistrictComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfDistrictComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::District) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::District) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::District> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::District> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfFloorComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Floor>,
}

impl AsMut<AddressArrayOfFloorComponent> for AddressArrayOfFloorComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfFloorComponent> for AddressArrayOfFloorComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfFloorComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfFloorComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfFloorComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Floor) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Floor) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Floor> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Floor> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<AddressArrayOfIDComponent> for AddressArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfIDComponent> for AddressArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfIDComponent {
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
pub struct AddressArrayOfInhouseMailComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::InhouseMail>,
}

impl AsMut<AddressArrayOfInhouseMailComponent> for AddressArrayOfInhouseMailComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfInhouseMailComponent> for AddressArrayOfInhouseMailComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfInhouseMailComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfInhouseMailComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfInhouseMailComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::InhouseMail) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::InhouseMail) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::InhouseMail> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::InhouseMail> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfLocationCoordinateComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LocationCoordinate>,
}

impl AsMut<AddressArrayOfLocationCoordinateComponent> for AddressArrayOfLocationCoordinateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfLocationCoordinateComponent> for AddressArrayOfLocationCoordinateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfLocationCoordinateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfLocationCoordinateComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::LocationCoordinate) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::LocationCoordinate) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::LocationCoordinate> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::LocationCoordinate> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfMarkAttentionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MarkAttention>,
}

impl AsMut<AddressArrayOfMarkAttentionComponent> for AddressArrayOfMarkAttentionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfMarkAttentionComponent> for AddressArrayOfMarkAttentionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfMarkAttentionComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfMarkAttentionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfMarkAttentionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MarkAttention) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MarkAttention) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MarkAttention> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MarkAttention> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfMarkCareComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::MarkCare>,
}

impl AsMut<AddressArrayOfMarkCareComponent> for AddressArrayOfMarkCareComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfMarkCareComponent> for AddressArrayOfMarkCareComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfMarkCareComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfMarkCareComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfMarkCareComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::MarkCare) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::MarkCare) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::MarkCare> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::MarkCare> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfPlotIdentificationComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PlotIdentification>,
}

impl AsMut<AddressArrayOfPlotIdentificationComponent> for AddressArrayOfPlotIdentificationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfPlotIdentificationComponent> for AddressArrayOfPlotIdentificationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfPlotIdentificationComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfPlotIdentificationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfPlotIdentificationComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PlotIdentification) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PlotIdentification) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PlotIdentification> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PlotIdentification> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfPostalZoneComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::PostalZone>,
}

impl AsMut<AddressArrayOfPostalZoneComponent> for AddressArrayOfPostalZoneComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfPostalZoneComponent> for AddressArrayOfPostalZoneComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfPostalZoneComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfPostalZoneComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfPostalZoneComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::PostalZone) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::PostalZone) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::PostalZone> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::PostalZone> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfPostboxComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Postbox>,
}

impl AsMut<AddressArrayOfPostboxComponent> for AddressArrayOfPostboxComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfPostboxComponent> for AddressArrayOfPostboxComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfPostboxComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfPostboxComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfPostboxComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Postbox) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Postbox) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Postbox> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Postbox> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfRegionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Region>,
}

impl AsMut<AddressArrayOfRegionComponent> for AddressArrayOfRegionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfRegionComponent> for AddressArrayOfRegionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfRegionComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfRegionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfRegionComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Region) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Region) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Region> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Region> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfRoomComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Room>,
}

impl AsMut<AddressArrayOfRoomComponent> for AddressArrayOfRoomComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfRoomComponent> for AddressArrayOfRoomComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfRoomComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfRoomComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfRoomComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Room) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Room) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Room> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Room> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfStreetNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::StreetName>,
}

impl AsMut<AddressArrayOfStreetNameComponent> for AddressArrayOfStreetNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfStreetNameComponent> for AddressArrayOfStreetNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfStreetNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfStreetNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfStreetNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::StreetName) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::StreetName) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::StreetName> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::StreetName> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AddressArrayOfTimezoneOffsetComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::TimezoneOffset>,
}

impl AsMut<AddressArrayOfTimezoneOffsetComponent> for AddressArrayOfTimezoneOffsetComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<AddressArrayOfTimezoneOffsetComponent> for AddressArrayOfTimezoneOffsetComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("AddressArrayOfTimezoneOffsetComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("AddressArrayOfTimezoneOffsetComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl AddressArrayOfTimezoneOffsetComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::TimezoneOffset) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::TimezoneOffset) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::TimezoneOffset> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::TimezoneOffset> {
        self.items.iter()
    }
}

