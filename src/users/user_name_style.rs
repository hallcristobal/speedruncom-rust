use remove_string_sub;
use serde_json::Value;

pub struct UserNameStyle {
    is_gradient: bool,
    light_solid_color_code: Option<String>,
    light_gradient_start_color_code: Option<String>,
    light_gradient_end_color_code: Option<String>,
    dark_solid_color_code: Option<String>,
    dark_gradient_start_color_code: Option<String>,
    dark_gradient_end_color_code: Option<String>,
}

impl UserNameStyle {
    pub fn parse(object: &mut Value) -> Option<Self> {
        if let Some(object) = object.as_object_mut() {
            let is_gradient = object["style"].as_str().unwrap() == "gradient";
            if is_gradient {
                let style = UserNameStyle {
                    is_gradient: is_gradient,
                    light_solid_color_code: None,
                    light_gradient_start_color_code: remove_string_sub("light",
                                                                       &mut object["color-from"]),
                    light_gradient_end_color_code: remove_string_sub("light",
                                                                     &mut object["color-to"]),
                    dark_solid_color_code: None,
                    dark_gradient_start_color_code: remove_string_sub("dark",
                                                                      &mut object["color-from"]),
                    dark_gradient_end_color_code: remove_string_sub("dark",
                                                                    &mut object["color-to"]),
                };
                Some(style)
            } else {
                let style = UserNameStyle {
                    is_gradient: is_gradient,
                    light_solid_color_code: remove_string_sub("light", &mut object["color"]),
                    light_gradient_start_color_code: None,
                    light_gradient_end_color_code: None,
                    dark_solid_color_code: remove_string_sub("dark", &mut object["color"]),
                    dark_gradient_start_color_code: None,
                    dark_gradient_end_color_code: None,
                };
                Some(style)
            }
        } else {
            None
        }
    }

    pub fn is_gradient(&self) -> bool {
        self.is_gradient
    }

    pub fn get_light_solid_color_code(&self) -> &Option<String> {
        &self.light_solid_color_code
    }

    pub fn get_light_gradient_start_color_code(&self) -> &Option<String> {
        &self.light_gradient_start_color_code
    }

    pub fn get_light_gradient_end_color_code(&self) -> &Option<String> {
        &self.light_gradient_end_color_code
    }

    pub fn get_dark_solid_color_code(&self) -> &Option<String> {
        &self.dark_solid_color_code
    }

    pub fn get_dark_gradient_start_color_code(&self) -> &Option<String> {
        &self.dark_gradient_start_color_code
    }

    pub fn get_dark_gradient_end_color_code(&self) -> &Option<String> {
        &self.dark_gradient_end_color_code
    }
}
