use serde::Serialize;

// Reference: https://github.com/SteelSeries/gamesense-sdk/blob/master/doc/api/json-handlers-screen.md


#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct FrameModifiersData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length_millis: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeats: Option<isize>
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct DataAccessorData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_frame_key: Option<String>
}


#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct TextModifiersData {
    pub has_text: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap: Option<isize>
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ProgressBarData {
    pub has_progress_bar: bool
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum LineDataType {
    TextModifiersData(TextModifiersData),
    ProgressBarData(ProgressBarData)
}


#[derive(Serialize, Debug)]
pub struct LineData {
    #[serde(flatten)]
    pub type_options: LineDataType,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_accessor_data: Option<DataAccessorData>
}


#[derive(Serialize, Debug)]
pub struct SingleLineFrameData {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_modifiers_data: Option<FrameModifiersData>,
    #[serde(flatten)]
    pub line_data: LineData
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct MultiLineFrameData {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_modifiers_data: Option<FrameModifiersData>,
    pub lines: Vec<LineData>
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ImageFrameData {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_modifiers_data: Option<FrameModifiersData>,
    pub image_data: Vec<u8>
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum ScreenFrameData {
    SingleLineFrameData(SingleLineFrameData),
    MultiLineFrameData(MultiLineFrameData),
    ImageFrameData(ImageFrameData)
}


#[derive(Serialize, Debug)]
pub struct StaticScreenDataDefinition(pub Vec<ScreenFrameData>);

#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct RangeScreenDataDefintion {
    pub low: isize,
    pub high: isize,
    pub datas: StaticScreenDataDefinition
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum ScreenDataDefinition {
    StaticScreenDataDefinition(StaticScreenDataDefinition),
    RangeScreenDataDefintion(RangeScreenDataDefintion)
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ScreenHandler {
    pub device_type: String,
    pub zone: String,
    mode: String,
    pub datas: ScreenDataDefinition
}

impl ScreenHandler {
    pub fn new(device_type: &str, zone: &str, datas: ScreenDataDefinition) -> ScreenHandler {
        ScreenHandler {
            device_type: device_type.to_owned(),
            zone: zone.to_owned(),
            mode: String::from("screen"),
            datas: datas
        }
    }
}
