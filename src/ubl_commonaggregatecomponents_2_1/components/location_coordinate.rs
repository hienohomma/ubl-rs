use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LocationCoordinate {
    #[serde(rename = "AltitudeMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude_measure: Option<LocationCoordinateArrayOfAltitudeMeasureComponent>,
    #[serde(rename = "CoordinateSystemCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate_system_code: Option<LocationCoordinateArrayOfCoordinateSystemCodeComponent>,
    #[serde(rename = "LatitudeDegreesMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude_degrees_measure: Option<LocationCoordinateArrayOfLatitudeDegreesMeasureComponent>,
    #[serde(rename = "LatitudeDirectionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude_direction_code: Option<LocationCoordinateArrayOfLatitudeDirectionCodeComponent>,
    #[serde(rename = "LatitudeMinutesMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude_minutes_measure: Option<LocationCoordinateArrayOfLatitudeMinutesMeasureComponent>,
    #[serde(rename = "LongitudeDegreesMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude_degrees_measure: Option<LocationCoordinateArrayOfLongitudeDegreesMeasureComponent>,
    #[serde(rename = "LongitudeDirectionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude_direction_code: Option<LocationCoordinateArrayOfLongitudeDirectionCodeComponent>,
    #[serde(rename = "LongitudeMinutesMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude_minutes_measure: Option<LocationCoordinateArrayOfLongitudeMinutesMeasureComponent>,
}

impl AsMut<LocationCoordinate> for LocationCoordinate {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LocationCoordinate> for LocationCoordinate {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Some(v) = &self.latitude_minutes_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LocationCoordinate.latitude_minutes_measure", e));
            }
        }
        if let Some(v) = &self.longitude_minutes_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LocationCoordinate.longitude_minutes_measure", e));
            }
        }
        if let Some(v) = &self.longitude_direction_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LocationCoordinate.longitude_direction_code", e));
            }
        }
        if let Some(v) = &self.latitude_degrees_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LocationCoordinate.latitude_degrees_measure", e));
            }
        }
        if let Some(v) = &self.longitude_degrees_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LocationCoordinate.longitude_degrees_measure", e));
            }
        }
        if let Some(v) = &self.altitude_measure {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LocationCoordinate.altitude_measure", e));
            }
        }
        if let Some(v) = &self.coordinate_system_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LocationCoordinate.coordinate_system_code", e));
            }
        }
        if let Some(v) = &self.latitude_direction_code {
            if let Err(e) = v.validate() {
                return Err(UblError::optional_component("LocationCoordinate.latitude_direction_code", e));
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

impl LocationCoordinate {
    pub fn title() -> &'static str {
        "Location Coordinate. Details"
    }
    pub fn description() -> &'static str {
        "A class for defining a set of geographical coordinates (apparently misnamed)."
    }
    pub fn new() -> Component<Self> {
        Component(Self {
            altitude_measure: None,
            longitude_minutes_measure: None,
            latitude_direction_code: None,
            latitude_minutes_measure: None,
            coordinate_system_code: None,
            latitude_degrees_measure: None,
            longitude_degrees_measure: None,
            longitude_direction_code: None,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LocationCoordinateArrayOfAltitudeMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::AltitudeMeasure>,
}

impl AsMut<LocationCoordinateArrayOfAltitudeMeasureComponent> for LocationCoordinateArrayOfAltitudeMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LocationCoordinateArrayOfAltitudeMeasureComponent> for LocationCoordinateArrayOfAltitudeMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LocationCoordinateArrayOfAltitudeMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LocationCoordinateArrayOfAltitudeMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LocationCoordinateArrayOfAltitudeMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::AltitudeMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::AltitudeMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::AltitudeMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::AltitudeMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LocationCoordinateArrayOfCoordinateSystemCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::CoordinateSystemCode>,
}

impl AsMut<LocationCoordinateArrayOfCoordinateSystemCodeComponent> for LocationCoordinateArrayOfCoordinateSystemCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LocationCoordinateArrayOfCoordinateSystemCodeComponent> for LocationCoordinateArrayOfCoordinateSystemCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LocationCoordinateArrayOfCoordinateSystemCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LocationCoordinateArrayOfCoordinateSystemCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LocationCoordinateArrayOfCoordinateSystemCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::CoordinateSystemCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::CoordinateSystemCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::CoordinateSystemCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::CoordinateSystemCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LocationCoordinateArrayOfLatitudeDegreesMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LatitudeDegreesMeasure>,
}

impl AsMut<LocationCoordinateArrayOfLatitudeDegreesMeasureComponent> for LocationCoordinateArrayOfLatitudeDegreesMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LocationCoordinateArrayOfLatitudeDegreesMeasureComponent> for LocationCoordinateArrayOfLatitudeDegreesMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LocationCoordinateArrayOfLatitudeDegreesMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LocationCoordinateArrayOfLatitudeDegreesMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LocationCoordinateArrayOfLatitudeDegreesMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LatitudeDegreesMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LatitudeDegreesMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LatitudeDegreesMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LatitudeDegreesMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LocationCoordinateArrayOfLatitudeDirectionCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LatitudeDirectionCode>,
}

impl AsMut<LocationCoordinateArrayOfLatitudeDirectionCodeComponent> for LocationCoordinateArrayOfLatitudeDirectionCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LocationCoordinateArrayOfLatitudeDirectionCodeComponent> for LocationCoordinateArrayOfLatitudeDirectionCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LocationCoordinateArrayOfLatitudeDirectionCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LocationCoordinateArrayOfLatitudeDirectionCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LocationCoordinateArrayOfLatitudeDirectionCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LatitudeDirectionCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LatitudeDirectionCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LatitudeDirectionCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LatitudeDirectionCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LocationCoordinateArrayOfLatitudeMinutesMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LatitudeMinutesMeasure>,
}

impl AsMut<LocationCoordinateArrayOfLatitudeMinutesMeasureComponent> for LocationCoordinateArrayOfLatitudeMinutesMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LocationCoordinateArrayOfLatitudeMinutesMeasureComponent> for LocationCoordinateArrayOfLatitudeMinutesMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LocationCoordinateArrayOfLatitudeMinutesMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LocationCoordinateArrayOfLatitudeMinutesMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LocationCoordinateArrayOfLatitudeMinutesMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LatitudeMinutesMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LatitudeMinutesMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LatitudeMinutesMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LatitudeMinutesMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LocationCoordinateArrayOfLongitudeDegreesMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LongitudeDegreesMeasure>,
}

impl AsMut<LocationCoordinateArrayOfLongitudeDegreesMeasureComponent> for LocationCoordinateArrayOfLongitudeDegreesMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LocationCoordinateArrayOfLongitudeDegreesMeasureComponent> for LocationCoordinateArrayOfLongitudeDegreesMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LocationCoordinateArrayOfLongitudeDegreesMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LocationCoordinateArrayOfLongitudeDegreesMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LocationCoordinateArrayOfLongitudeDegreesMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LongitudeDegreesMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LongitudeDegreesMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LongitudeDegreesMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LongitudeDegreesMeasure> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LocationCoordinateArrayOfLongitudeDirectionCodeComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LongitudeDirectionCode>,
}

impl AsMut<LocationCoordinateArrayOfLongitudeDirectionCodeComponent> for LocationCoordinateArrayOfLongitudeDirectionCodeComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LocationCoordinateArrayOfLongitudeDirectionCodeComponent> for LocationCoordinateArrayOfLongitudeDirectionCodeComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LocationCoordinateArrayOfLongitudeDirectionCodeComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LocationCoordinateArrayOfLongitudeDirectionCodeComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LocationCoordinateArrayOfLongitudeDirectionCodeComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LongitudeDirectionCode) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LongitudeDirectionCode) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LongitudeDirectionCode> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LongitudeDirectionCode> {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LocationCoordinateArrayOfLongitudeMinutesMeasureComponent {
    pub items: Vec<crate::ubl_commonbasiccomponents_2_1::LongitudeMinutesMeasure>,
}

impl AsMut<LocationCoordinateArrayOfLongitudeMinutesMeasureComponent> for LocationCoordinateArrayOfLongitudeMinutesMeasureComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<LocationCoordinateArrayOfLongitudeMinutesMeasureComponent> for LocationCoordinateArrayOfLongitudeMinutesMeasureComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len > 1 {
            return Err(UblError::inner_component("LocationCoordinateArrayOfLongitudeMinutesMeasureComponent", format!("Max allowed items is 1 and you have {}", len)))
        }
        if len < 1 {
            return Err(UblError::inner_component("LocationCoordinateArrayOfLongitudeMinutesMeasureComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl LocationCoordinateArrayOfLongitudeMinutesMeasureComponent {
    pub fn new(item: crate::ubl_commonbasiccomponents_2_1::LongitudeMinutesMeasure) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonbasiccomponents_2_1::LongitudeMinutesMeasure) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonbasiccomponents_2_1::LongitudeMinutesMeasure> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonbasiccomponents_2_1::LongitudeMinutesMeasure> {
        self.items.iter()
    }
}

