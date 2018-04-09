/*
 * IotHub Gateway Service APIs
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: Service
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigurationContent {
    #[serde(rename = "ModulesContent", skip_serializing_if = "Option::is_none")]
    modules_content: Option<::std::collections::HashMap<String, Vec<::models::TwinContent>>>,
    #[serde(rename = "DeviceContent", skip_serializing_if = "Option::is_none")]
    device_content: Option<Vec<::models::TwinContent>>,
    #[serde(rename = "ModuleContent", skip_serializing_if = "Option::is_none")]
    module_content: Option<Vec<::models::TwinContent>>,
    #[serde(rename = "IsEdgeContent", skip_serializing_if = "Option::is_none")]
    is_edge_content: Option<bool>,
}

impl ConfigurationContent {
    pub fn new() -> ConfigurationContent {
        ConfigurationContent {
            modules_content: None,
            device_content: None,
            module_content: None,
            is_edge_content: None,
        }
    }

    pub fn set_modules_content(
        &mut self,
        modules_content: ::std::collections::HashMap<String, Vec<::models::TwinContent>>,
    ) {
        self.modules_content = Some(modules_content);
    }

    pub fn with_modules_content(
        mut self,
        modules_content: ::std::collections::HashMap<String, Vec<::models::TwinContent>>,
    ) -> ConfigurationContent {
        self.modules_content = Some(modules_content);
        self
    }

    pub fn modules_content(
        &self,
    ) -> Option<&::std::collections::HashMap<String, Vec<::models::TwinContent>>> {
        self.modules_content.as_ref()
    }

    pub fn reset_modules_content(&mut self) {
        self.modules_content = None;
    }

    pub fn set_device_content(&mut self, device_content: Vec<::models::TwinContent>) {
        self.device_content = Some(device_content);
    }

    pub fn with_device_content(
        mut self,
        device_content: Vec<::models::TwinContent>,
    ) -> ConfigurationContent {
        self.device_content = Some(device_content);
        self
    }

    pub fn device_content(&self) -> Option<&Vec<::models::TwinContent>> {
        self.device_content.as_ref()
    }

    pub fn reset_device_content(&mut self) {
        self.device_content = None;
    }

    pub fn set_module_content(&mut self, module_content: Vec<::models::TwinContent>) {
        self.module_content = Some(module_content);
    }

    pub fn with_module_content(
        mut self,
        module_content: Vec<::models::TwinContent>,
    ) -> ConfigurationContent {
        self.module_content = Some(module_content);
        self
    }

    pub fn module_content(&self) -> Option<&Vec<::models::TwinContent>> {
        self.module_content.as_ref()
    }

    pub fn reset_module_content(&mut self) {
        self.module_content = None;
    }

    pub fn set_is_edge_content(&mut self, is_edge_content: bool) {
        self.is_edge_content = Some(is_edge_content);
    }

    pub fn with_is_edge_content(mut self, is_edge_content: bool) -> ConfigurationContent {
        self.is_edge_content = Some(is_edge_content);
        self
    }

    pub fn is_edge_content(&self) -> Option<&bool> {
        self.is_edge_content.as_ref()
    }

    pub fn reset_is_edge_content(&mut self) {
        self.is_edge_content = None;
    }
}