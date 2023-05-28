use cursive::theme::BaseColor::*;
use cursive::theme::Color::TerminalDefault;
use cursive::theme::Effect::*;
use cursive::theme::PaletteColor::*;
use cursive::theme::Style;
use cursive::theme::{BorderStyle, Palette, Theme};
use cursive::traits::With;
use cursive::views::{Dialog, TextView};
use cursive::Cursive;

pub enum TUIThemes {
    Default,
    DarkSolarized,
    LightSolarized,
    Dracula,
}

impl TUIThemes {
    pub fn get_theme(&self) -> Theme {
        match self {
            TUIThemes::Default => cursive::theme::load_default(),
            TUIThemes::DarkSolarized => cursive::theme::Theme {
                shadow: true,
                borders: BorderStyle::Simple,
                palette: Palette::retro().with(|palette| {
                    {
                        palette[Background] = TerminalDefault;
                        palette[View] = TerminalDefault;
                        palette[Primary] = White.dark();
                        palette[TitlePrimary] = Blue.light();
                        palette[Secondary] = Blue.light();
                        palette[Highlight] = Blue.dark();
                    }

                    {
                        use cursive::theme::PaletteStyle::*;
                        palette[Highlight] = Style::from(Blue.light()).combine(Bold);
                    }
                }),
            },
            TUIThemes::LightSolarized => cursive::theme::Theme {
                shadow: false,
                borders: BorderStyle::Simple,
                palette: Palette::retro().with(|palette| {
                    use cursive::theme::BaseColor::*;

                    {
                        palette[Background] = TerminalDefault;
                        palette[View] = TerminalDefault;
                        palette[Primary] = Cyan.dark();
                        palette[TitlePrimary] = Yellow.light();
                        palette[Secondary] = Yellow.light();
                        palette[Highlight] = Cyan.dark();
                    }

                    {
                        // Then override some styles.
                        use cursive::theme::PaletteStyle::*;
                        palette[Highlight] = Style::from(Yellow.light()).combine(Bold);
                    }
                }),
            },
            TUIThemes::Dracula => cursive::theme::Theme {
                shadow: false,
                borders: BorderStyle::Simple,
                palette: Palette::retro().with(|palette| {
                    use cursive::theme::BaseColor::*;

                    {
                        palette[Background] = TerminalDefault;
                        palette[View] = TerminalDefault;
                        palette[Primary] = Magenta.dark();
                        palette[TitlePrimary] = Cyan.light();
                        palette[Secondary] = Cyan.light();
                        palette[Highlight] = Magenta.dark();
                    }

                    {
                        use cursive::theme::PaletteStyle::*;
                        palette[Highlight] = Style::from(Cyan.light()).combine(Bold);
                    }
                }),
            },
        }
    }
}

use super::draw::TUIDisplay;
pub fn popup_theme_menu(s: &mut Cursive) {
    s.pop_layer();
    let state = s
        .user_data::<crate::bin_modules::UserData>()
        .unwrap()
        .clone()
        .state;
    s.add_layer(
        Dialog::around(TextView::new("Choose a theme"))
            .title("Theme")
            .button("Default", move |s| {
                s.set_theme(TUIThemes::Default.get_theme());
            })
            .button("Dark Solarized", move |s| {
                s.set_theme(TUIThemes::DarkSolarized.get_theme());
            })
            .button("Light Solarized", move |s| {
                s.set_theme(TUIThemes::LightSolarized.get_theme());
            })
            .button("Dracula", move |s| {
                s.set_theme(TUIThemes::Dracula.get_theme());
            })
            .button("Back", move |s| {
                s.pop_layer();
                state.draw(s);
            }),
    );
}
