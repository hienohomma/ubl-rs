use serde::{Deserialize, Serialize};
use crate::{UblError, Component, Componentable};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UBLExtensions {
    #[serde(rename = "UBLExtension")]
    pub ublextension: UBLExtensionsArrayOfUBLExtensionComponent,
}

impl AsMut<UBLExtensions> for UBLExtensions {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UBLExtensions> for UBLExtensions {
    fn validate(&self) -> Result<&Self, UblError> {
        if let Err(e) = self.ublextension.validate() {
            return Err(UblError::component("UBLExtensions.ublextension", e));
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

impl UBLExtensions {
    pub fn description() -> &'static str {
        "A container for all extensions present in the document."
    }
    pub fn new(ublextension: UBLExtensionsArrayOfUBLExtensionComponent) -> Component<Self> {
        Component(Self {
            ublextension,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UBLExtensionsArrayOfUBLExtensionComponent {
    pub items: Vec<crate::ubl_commonextensioncomponents_2_1::UBLExtension>,
}

impl AsMut<UBLExtensionsArrayOfUBLExtensionComponent> for UBLExtensionsArrayOfUBLExtensionComponent {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Componentable<UBLExtensionsArrayOfUBLExtensionComponent> for UBLExtensionsArrayOfUBLExtensionComponent {
    fn validate(&self) -> Result<&Self, UblError> {
        let len = self.items.len();

        if len < 1 {
            return Err(UblError::inner_component("UBLExtensionsArrayOfUBLExtensionComponent", format!("Min allowed items is 1 and you have {}", len)))
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

impl UBLExtensionsArrayOfUBLExtensionComponent {
    pub fn new(item: crate::ubl_commonextensioncomponents_2_1::UBLExtension) -> Component<Self> {
        Component(Self {
            items: vec![item],
        })
    }
    pub fn push(&mut self, item: crate::ubl_commonextensioncomponents_2_1::UBLExtension) {
        self.items.push(item);
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<crate::ubl_commonextensioncomponents_2_1::UBLExtension> {
        self.items.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<crate::ubl_commonextensioncomponents_2_1::UBLExtension> {
        self.items.iter()
    }
}

