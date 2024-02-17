use cursive::theme::{BorderStyle, Color, Palette, PaletteColor, Theme};

pub fn get_theme() -> Theme {
    let mut palette = Palette::default();

    palette[PaletteColor::Background] = Color::parse("#000000").unwrap();
    palette[PaletteColor::Shadow] = Color::parse("#000000").unwrap();
    palette[PaletteColor::View] = Color::parse("#000000").unwrap();

    palette[PaletteColor::Primary] = Color::parse("#fff").unwrap();
    palette[PaletteColor::Secondary] = Color::parse("#c1c1c1").unwrap();
    palette[PaletteColor::Tertiary] = Color::parse("#000000").unwrap();

    palette[PaletteColor::TitlePrimary] = Color::parse("#5555FF").unwrap();
    palette[PaletteColor::TitleSecondary] = Color::parse("#FFFF00").unwrap();

    palette[PaletteColor::Highlight] = Color::parse("#ff00ff").unwrap();
    palette[PaletteColor::HighlightInactive] = Color::parse("#5555FF").unwrap();

    Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette,
    }
}
