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
}
