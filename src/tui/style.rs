use cursive::theme::{BorderStyle, Palette, Theme};

pub fn get_theme() -> Theme {
    let mut theme = Theme::default();

    theme.shadow = false;
    theme.borders = BorderStyle::Simple;

    let palette = Palette::terminal_default();

    // palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);

    theme.palette = palette;

    theme
}
