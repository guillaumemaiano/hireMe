use eframe::egui::{FontData, FontDefinitions, FontFamily};

pub fn setup_fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();

    // Primary Latin font: CrimsonText
    fonts.font_data.insert(
        "CrimsonText".to_owned(),
        FontData::from_owned(
            include_bytes!("../assets/CrimsonText-Regular.ttf").to_vec()
        ).into(),
    );

    // For CJK
    fonts.font_data.insert(
    "NotoSansCJK".to_owned(),
    FontData::from_owned(
        include_bytes!("../assets/NotoSansSC-Regular.ttf").to_vec()
    ).into(),
);


    // Fallback font: NotoSans (covers Cyrillic)
    fonts.font_data.insert(
        "NotoSans".to_owned(),
        FontData::from_owned(
            include_bytes!("../assets/NotoSans-Regular.ttf").to_vec()
        ).into(),
    );

    // Proportional: Crimson first, then Noto
    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
        .insert(0, "CrimsonText".to_owned());
    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
        .push("NotoSans".to_owned());
    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
    .push("NotoSansCJK".to_owned());

    fonts.families.get_mut(&FontFamily::Monospace).unwrap()
        .insert(0, "NotoSans".to_owned());

    fonts
}
