use crate::traits::array_enum::ArrayEnum;

pub enum BodyTypeEnum {
    Json,
    Xml,
    None,
}

impl ArrayEnum for BodyTypeEnum {
    fn get_next(&self) -> Self {
        match self {
            BodyTypeEnum::Json => BodyTypeEnum::Xml,
            BodyTypeEnum::Xml => BodyTypeEnum::None,
            BodyTypeEnum::None => BodyTypeEnum::Json,
        }
    }

    fn get_prev(&self) -> Self {
        match self {
            BodyTypeEnum::Json => BodyTypeEnum::None,
            BodyTypeEnum::Xml => BodyTypeEnum::Json,
            BodyTypeEnum::None => BodyTypeEnum::Xml,
        }
    }

    fn get_array_form() -> Vec<Self> {
        vec![BodyTypeEnum::Json, BodyTypeEnum::Xml, BodyTypeEnum::None]
    }
}

impl ToString for BodyTypeEnum {
    fn to_string(&self) -> String {
        match self {
            BodyTypeEnum::None => "None",
            BodyTypeEnum::Json => "JSON",
            BodyTypeEnum::Xml => "XML",
        }
        .into()
    }
}
