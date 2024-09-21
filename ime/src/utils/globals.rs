use windows::core::GUID;

pub const CLSID_PREFIX: &str = "CLSID\\";
pub const INPROC_SUFFIX: &str = "\\InProcServer32";

pub const SERVICE_NAME: &str = "Azookey";

// ffdefe79-2fc2-11ef-b16b-94e70b2c378c
pub const GUID_TEXT_SERVICE: GUID = GUID::from_u128(0xffdefe79_2fc2_11ef_b16b_94e70b2c378c);
// ffdefe7a-2fc2-11ef-b16b-94e70b2c378c
pub const GUID_PROFILE: GUID = GUID::from_u128(0xffdefe7a_2fc2_11ef_b16b_94e70b2c378c);

// DisplayAttribute用のGUID3つ
pub const GUID_DISPLAY_ATTRIBUTE_INPUT: GUID =
    GUID::from_u128(0xffdefe7b_2fc2_11ef_b16b_94e70b2c378c);
pub const GUID_DISPLAY_ATTRIBUTE_CONVERTED: GUID =
    GUID::from_u128(0xffdefe7c_2fc2_11ef_b16b_94e70b2c378c);
pub const GUID_DISPLAY_ATTRIBUTE_FOCUSED: GUID =
    GUID::from_u128(0xffdefe7d_2fc2_11ef_b16b_94e70b2c378c);

// これはなんだろう..?
// https://github.com/microsoft/Windows-classic-samples/blob/main/Samples/Win7Samples/winui/input/tsf/textservice/textservice-step04/LanguageBar.cpp#L23
pub const TEXTSERVICE_LANGBARITEMSINK_COOKIE: u32 = 0x0fab0fab;
