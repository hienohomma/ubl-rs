use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Location {
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<LocationArrayOfAddressComponent>,
    #[serde(rename = "Conditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<LocationArrayOfConditionsComponent>,
    #[serde(rename = "CountrySubentity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_subentity: Option<LocationArrayOfCountrySubentityComponent>,
    #[serde(rename = "CountrySubentityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_subentity_code: Option<LocationArrayOfCountrySubentityCodeComponent>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<LocationArrayOfDescriptionComponent>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<LocationArrayOfIDComponent>,
    #[serde(rename = "InformationURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub information_uri: Option<LocationArrayOfInformationURIComponent>,
    #[serde(rename = "LocationCoordinate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_coordinate: Option<LocationArrayOfLocationCoordinateComponent>,
    #[serde(rename = "LocationTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_type_code: Option<LocationArrayOfLocationTypeCodeComponent>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<LocationArrayOfNameComponent>,
    #[serde(rename = "SubsidiaryLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsidiary_location: Option<LocationArrayOfSubsidiaryLocationComponent>,
    #[serde(rename = "ValidityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<LocationArrayOfValidityPeriodComponent>,
}

impl AsMut<Location> for Location {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<Location> for Location {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.information_uri {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Location.information_uri", e));
            }
        }
        if let Some(v) = &self.location_coordinate {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Location.location_coordinate", e));
            }
        }
        if let Some(v) = &self.subsidiary_location {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Location.subsidiary_location", e));
            }
        }
        if let Some(v) = &self.address {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Location.address", e));
            }
        }
        if let Some(v) = &self.name {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Location.name", e));
            }
        }
        if let Some(v) = &self.id {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Location.id", e));
            }
        }
        if let Some(v) = &self.country_subentity_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Location.country_subentity_code", e));
            }
        }
        if let Some(v) = &self.location_type_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Location.location_type_code", e));
            }
        }
        if let Some(v) = &self.country_subentity {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Location.country_subentity", e));
            }
        }
        if let Some(v) = &self.validity_period {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Location.validity_period", e));
            }
        }
        if let Some(v) = &self.description {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Location.description", e));
            }
        }
        if let Some(v) = &self.conditions {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("Location.conditions", e));
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

impl Location {
    pub fn title() -> &'static str {
        "Location. Details"
    }
    pub fn description() -> &'static str {
        "A class to describe a location."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            country_subentity_code: None,
            conditions: None,
            description: None,
            location_type_code: None,
            country_subentity: None,
            validity_period: None,
            location_coordinate: None,
            name: None,
            address: None,
            id: None,
            information_uri: None,
            subsidiary_location: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LocationArrayOfAddressComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::Address>,
}

impl AsMut<LocationArrayOfAddressComponent> for LocationArrayOfAddressComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LocationArrayOfAddressComponent> for LocationArrayOfAddressComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LocationArrayOfAddressComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LocationArrayOfAddressComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LocationArrayOfAddressComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::Address) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::Address) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::Address> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::Address> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LocationArrayOfConditionsComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Conditions>,
}

impl AsMut<LocationArrayOfConditionsComponent> for LocationArrayOfConditionsComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LocationArrayOfConditionsComponent> for LocationArrayOfConditionsComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("LocationArrayOfConditionsComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LocationArrayOfConditionsComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Conditions) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Conditions) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Conditions> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Conditions> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LocationArrayOfCountrySubentityComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CountrySubentity>,
}

impl AsMut<LocationArrayOfCountrySubentityComponent> for LocationArrayOfCountrySubentityComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LocationArrayOfCountrySubentityComponent> for LocationArrayOfCountrySubentityComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LocationArrayOfCountrySubentityComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LocationArrayOfCountrySubentityComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LocationArrayOfCountrySubentityComponent {
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
pub struct LocationArrayOfCountrySubentityCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CountrySubentityCode>,
}

impl AsMut<LocationArrayOfCountrySubentityCodeComponent> for LocationArrayOfCountrySubentityCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LocationArrayOfCountrySubentityCodeComponent> for LocationArrayOfCountrySubentityCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LocationArrayOfCountrySubentityCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LocationArrayOfCountrySubentityCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LocationArrayOfCountrySubentityCodeComponent {
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
pub struct LocationArrayOfDescriptionComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Description>,
}

impl AsMut<LocationArrayOfDescriptionComponent> for LocationArrayOfDescriptionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LocationArrayOfDescriptionComponent> for LocationArrayOfDescriptionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("LocationArrayOfDescriptionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LocationArrayOfDescriptionComponent {
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
pub struct LocationArrayOfIDComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::ID>,
}

impl AsMut<LocationArrayOfIDComponent> for LocationArrayOfIDComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LocationArrayOfIDComponent> for LocationArrayOfIDComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LocationArrayOfIDComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LocationArrayOfIDComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LocationArrayOfIDComponent {
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
pub struct LocationArrayOfInformationURIComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::InformationURI>,
}

impl AsMut<LocationArrayOfInformationURIComponent> for LocationArrayOfInformationURIComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LocationArrayOfInformationURIComponent> for LocationArrayOfInformationURIComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LocationArrayOfInformationURIComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LocationArrayOfInformationURIComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LocationArrayOfInformationURIComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::InformationURI) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::InformationURI) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::InformationURI> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::InformationURI> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LocationArrayOfLocationCoordinateComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::LocationCoordinate>,
}

impl AsMut<LocationArrayOfLocationCoordinateComponent> for LocationArrayOfLocationCoordinateComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LocationArrayOfLocationCoordinateComponent> for LocationArrayOfLocationCoordinateComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("LocationArrayOfLocationCoordinateComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LocationArrayOfLocationCoordinateComponent {
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
pub struct LocationArrayOfLocationTypeCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LocationTypeCode>,
}

impl AsMut<LocationArrayOfLocationTypeCodeComponent> for LocationArrayOfLocationTypeCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LocationArrayOfLocationTypeCodeComponent> for LocationArrayOfLocationTypeCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LocationArrayOfLocationTypeCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LocationArrayOfLocationTypeCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LocationArrayOfLocationTypeCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LocationTypeCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LocationTypeCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LocationTypeCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LocationTypeCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LocationArrayOfNameComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::Name>,
}

impl AsMut<LocationArrayOfNameComponent> for LocationArrayOfNameComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LocationArrayOfNameComponent> for LocationArrayOfNameComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LocationArrayOfNameComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LocationArrayOfNameComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LocationArrayOfNameComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::Name) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::Name) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::Name> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::Name> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LocationArrayOfSubsidiaryLocationComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::SubsidiaryLocation>,
}

impl AsMut<LocationArrayOfSubsidiaryLocationComponent> for LocationArrayOfSubsidiaryLocationComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LocationArrayOfSubsidiaryLocationComponent> for LocationArrayOfSubsidiaryLocationComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("LocationArrayOfSubsidiaryLocationComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LocationArrayOfSubsidiaryLocationComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::SubsidiaryLocation) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::SubsidiaryLocation) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::SubsidiaryLocation> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::SubsidiaryLocation> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LocationArrayOfValidityPeriodComponent {
    pub items: Vec<crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod>,
}

impl AsMut<LocationArrayOfValidityPeriodComponent> for LocationArrayOfValidityPeriodComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LocationArrayOfValidityPeriodComponent> for LocationArrayOfValidityPeriodComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("LocationArrayOfValidityPeriodComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LocationArrayOfValidityPeriodComponent {
    pub fn new(item: crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonaggregatecomponents_2_1::ValidityPeriod> {
        self.items.iter()
    }
}

