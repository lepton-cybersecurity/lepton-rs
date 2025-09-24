use iced::{Color, Theme};
use iced::theme::Palette;

#[derive(Debug)]
pub enum LeptonColor {
    LeptonOrange,
    LeptonOrangeBright,
    LeptonOrangeDark,

    Valid,
    Info,
    Warn,
    Error,

    BackgroundMain,
    BackgroundAlt,
    SurfaceDark,
    SecondaryGrey,
    PrimaryGreyLight,

    Text,
}

impl LeptonColor {
    pub fn to_hex(&self) -> String {
        String::from(
            match self {
                LeptonColor::LeptonOrange => "#f44611",
                LeptonColor::LeptonOrangeBright => "#f66c42",
                LeptonColor::LeptonOrangeDark => "#c93609",

                LeptonColor::Valid => "#43b581",
                LeptonColor::Info => "#95acff",
                LeptonColor::Warn => "#faa61a",
                LeptonColor::Error => "#e06c75",

                LeptonColor::BackgroundMain => "#150e1a",
                LeptonColor::BackgroundAlt => "#221f26",
                LeptonColor::SurfaceDark => "#3a3340",
                LeptonColor::SecondaryGrey => "#7b6076",
                LeptonColor::PrimaryGreyLight => "#c09cc3",

                LeptonColor::Text => "#ffffff",
            }
        )
    }

    pub fn to_iced_color(&self) -> Color {
        match Color::parse(&self.to_hex()) {
            Some(c) => c,
            None => unreachable!("Color conversion failed for \"{}\" from hexadecimal &str to iced::Color", &self.to_hex())
        }
    }
}

fn lepton_iced_palette() -> Palette {
    Palette {
        background: LeptonColor::BackgroundMain.to_iced_color(),
        text: LeptonColor::Text.to_iced_color(),
        primary: LeptonColor::LeptonOrange.to_iced_color(),
        success: LeptonColor::Valid.to_iced_color(),
        danger: LeptonColor::Error.to_iced_color(),
    }
}

pub fn lepton_iced_theme() -> Theme {
    Theme::custom("LeptonTheme".to_string(), lepton_iced_palette())
}
