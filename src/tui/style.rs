use cursive::theme::{BaseColor, BorderStyle, Color, Palette, PaletteColor, Theme};

pub fn get_theme() -> Theme {
    let mut theme = Theme::default();

    theme.shadow = false;
    theme.borders = BorderStyle::Simple;

    let mut palette = Palette::default();

    palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::Shadow] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::View] = Color::Dark(BaseColor::Black);

    palette[PaletteColor::Primary] = Color::Dark(BaseColor::White);
    palette[PaletteColor::Secondary] = Color::parse("#c1c1c1").unwrap();
    palette[PaletteColor::Tertiary] = Color::Dark(BaseColor::Black);

    palette[PaletteColor::TitlePrimary] = Color::Dark(BaseColor::Blue);
    palette[PaletteColor::TitleSecondary] = Color::Dark(BaseColor::Yellow);

    palette[PaletteColor::Highlight] = Color::Dark(BaseColor::Magenta);
    palette[PaletteColor::HighlightInactive] = Color::parse("#5555FF").unwrap();

    theme.palette = palette;

    theme
}
