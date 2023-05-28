use cursive::{
    align::HAlign,
    event::EventResult,
    view::{Resizable, Scrollable},
    views::{Dialog, OnEventView, SelectView},
    Cursive,
};

use crate::{
    bin_modules::{AppState, UserData},
    CockHandler, FromString, GetVariants, InnerUser, Size, SizeType, ID,
};

/// [TUIDisplay] is a trait for drawing screens within the TUI.
/// The [TUIDisplay::draw] function takes a mutable reference to a [cursive::Cursive] instance as input and draws the screen.
pub trait TUIDisplay {
    /// Function to draw a screen within the TUI using the given [cursive::Cursive] instance.
    fn draw(&self, s: &mut cursive::Cursive);
}

/// [TUIDisplay] implementation for [AppState]
/// Repsonsible for drawing each screen.
impl TUIDisplay for AppState {
    fn draw(&self, s: &mut Cursive) {
        match self {
            AppState::Home => draw_home(s),
            AppState::Result => draw_result(s),
            AppState::Id => draw_id(s),
            AppState::Size => draw_size(s),
            AppState::Abnormalities => draw_manual_options(s),
            AppState::Curvature => draw_manual_options(s),
            AppState::Shape => draw_manual_options(s),
            _ => draw_options(s),
        }
    }
}

/// Function to draw the [AppState::Home] screen.
pub fn draw_home(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::around(
            cursive::views::TextView::new(
                "Welcome to Turn's Cock Tier Evaluator\n\n\
                Change theme at any time by pressing `\\`\n\n\
                ESC to Exit",
            )
            .h_align(HAlign::Center)
        )
        .title("Home Screen")
        .button("Begin", |s| {
            let mut val = s.user_data::<UserData>().unwrap().clone();
            val.state = val.state.next();
            s.set_user_data(val.clone());
            s.pop_layer();
            val.state.draw(s)
        }),
    )
}

/// Function to draw the [AppState::Result] screen.
pub fn draw_result(siv: &mut Cursive) {
    let val = siv.user_data::<UserData>().unwrap().clone();
    let handler = CockHandler::new(val.user, val.cock);
    siv.pop_layer();
    siv.add_layer(
        Dialog::text(format!("{handler}"))
            .title("COCK RESULTS")
            .button("Finish", Cursive::quit),
    )
}

/// Function to draw an [AppState] which doesnt require manual user input.
pub fn draw_options(siv: &mut Cursive) {
    let val = siv.user_data::<UserData>().unwrap().clone();
    let options = val.state.options();
    let title = val.state.clone();

    let mut select = SelectView::new().h_align(HAlign::Center).autojump();

    select.add_all_str(options);
    select.set_on_submit(move |s: &mut Cursive, item: &str| {
        let mut val = s.user_data::<UserData>().unwrap().clone();
        val.state = val.state.next();
        val.cock.from_str_part(title.as_str(), item);
        s.set_user_data(val.clone());
        s.pop_layer();
        val.state.draw(s)
    });

    // `j` and `k` keys for navigation
    let select = OnEventView::new(select)
        .on_pre_event_inner('k', |s, _| {
            let cb = s.select_up(1);
            Some(EventResult::Consumed(Some(cb)))
        })
        .on_pre_event_inner('j', |s, _| {
            let cb = s.select_down(1);
            Some(EventResult::Consumed(Some(cb)))
        });

    siv.add_layer(
        Dialog::around(select.scrollable().fixed_size((40, 20)))
            .title(title.as_str())
            .button("Prev", |s| {
                let mut val = s.user_data::<UserData>().unwrap().clone();
                val.state = val.state.prev();
                s.set_user_data(val.clone());
                s.pop_layer();
                val.state.draw(s)
            }),
    );
}

/// Function to draw the [AppState::Id] screen.
pub fn draw_id(siv: &mut Cursive) {
    let options = ID::get_variants();
    let mut select = SelectView::new().h_align(HAlign::Center).autojump();
    select.add_all_str(options);
    select.set_on_submit(move |s: &mut Cursive, item: &str| {
        s.pop_layer();
        if item == "User" {
            s.add_layer(
                Dialog::around(cursive::views::EditView::new().on_submit(|s, username| {
                    let username = username.to_string();
                    s.pop_layer();
                    s.add_layer(
                        Dialog::around(cursive::views::EditView::new().on_submit(
                            move |s, discord_username| {
                                let mut val = s.user_data::<UserData>().unwrap().clone();
                                val.user = ID::User(InnerUser {
                                    name: username.clone(),
                                    discord_name: discord_username.to_string(),
                                });
                                val.state = val.state.next();
                                s.set_user_data(val.clone());
                                s.pop_layer();
                                val.state.draw(s);
                            },
                        ))
                        .title("Input Discord Username"),
                    );
                }))
                .title("Input Username"),
            );
        } else {
            // For anonymous user, just advance to the next state
            let mut val = s.user_data::<UserData>().unwrap().clone();
            val.user = ID::Anonymous;
            val.state = val.state.next();
            s.set_user_data(val.clone());
            s.pop_layer();
            val.state.draw(s)
        }
    });

    // `j` and `k` keys for navigation
    let select = OnEventView::new(select)
        .on_pre_event_inner('k', |s, _| {
            let cb = s.select_up(1);
            Some(EventResult::Consumed(Some(cb)))
        })
        .on_pre_event_inner('j', |s, _| {
            let cb = s.select_down(1);
            Some(EventResult::Consumed(Some(cb)))
        });

    siv.add_layer(
        Dialog::around(select)
        .title("ID Type")
        .button("Back", |s| {
            let mut val = s.user_data::<UserData>().unwrap().clone();
            val.state = val.state.prev();
            s.set_user_data(val.clone());
            s.pop_layer();
            val.state.draw(s)
        }),
    );
}

/// Function to draw the [AppState::Size] screen.
pub fn draw_size(siv: &mut Cursive) {
    let options = SizeType::get_variants();
    let mut select = SelectView::new().h_align(HAlign::Center).autojump();
    select.add_all_str(options);
    select.set_on_submit(move |s: &mut Cursive, item: &str| {
        let size_type = SizeType::from_string(item);
        s.pop_layer();
        s.add_layer(
            Dialog::around(
                cursive::views::EditView::new()
                    .on_submit(move |s, length_input| {
                        let length = length_input.parse();
                        if length.is_err() {
                            s.pop_layer();
                            draw_error(s, "Please enter a valid number for length. (e.g. 5.5, 4, 3.1415)");
                            return;
                        }
                        let length: f32 = length.unwrap();
                        s.pop_layer();
                        s.add_layer(
                            Dialog::around(
                                cursive::views::EditView::new()
                                    .on_submit(move |s, girth_input| {
                                        let girth = girth_input.parse();
                                        if girth.is_err() {
                                            s.pop_layer();
                                            draw_error(s, "Please enter a valid number for girth. (e.g. 5.5, 4, 3.1415)");
                                            return;
                                        }
                                        let girth: f32 = girth.unwrap();
                                        s.pop_layer();
                                        let mut val = s.user_data::<UserData>().unwrap().clone();
                                        match size_type {
                                            SizeType::Centimeters => val.cock.size = Size::from_cm(length, girth),
                                            SizeType::Inches => val.cock.size = Size::from_in(length, girth),
                                        };
                                        val.state = val.state.next();
                                        s.set_user_data(val.clone());
                                        s.pop_layer();
                                        val.state.draw(s);
                                    })
                            ).title("Input Girth")
                        );
                    })
            ).title("Input Length")
        );
    });

    // `j` and `k` keys for navigation
    let select = OnEventView::new(select)
        .on_pre_event_inner('k', |s, _| {
            let cb = s.select_up(1);
            Some(EventResult::Consumed(Some(cb)))
        })
        .on_pre_event_inner('j', |s, _| {
            let cb = s.select_down(1);
            Some(EventResult::Consumed(Some(cb)))
        });

    siv.add_layer(
        Dialog::around(select)
        .title("Size Type")
        .button("Back", |s| {
            let mut val = s.user_data::<UserData>().unwrap().clone();
            val.state = val.state.prev();
            s.set_user_data(val.clone());
            s.pop_layer();
            val.state.draw(s)
        }),
    );
}

/// Function to draw an error popup with the given message, returning to the previous state when the "Ok" button is pressed.
pub fn draw_error(siv: &mut Cursive, error: &str) {
    siv.add_layer(Dialog::text(error).button("Ok", |s| {
        let val = s.user_data::<UserData>().unwrap().clone();
        s.pop_layer();
        val.state.draw(s)
    }))
}

/// Function to draw an [AppState] which may possibly require manual input of an option.
pub fn draw_manual_options(siv: &mut Cursive) {
    let state = siv.user_data::<UserData>().unwrap().state.clone();
    let options = state.options();

    let mut select = SelectView::new().h_align(HAlign::Center).autojump();
    select.add_all_str(options);
    select.set_on_submit(move |s, item: &str| {
        let i = item.clone().to_string();
        if item == "Other" || item == "Minor" || item == "Major" {
            s.add_layer(
                Dialog::around(cursive::views::EditView::new().on_submit(
                    move |s, custom_input| {
                        s.pop_layer();
                        let custom_option = custom_input.to_string();
                        let mut val = s.user_data::<UserData>().unwrap().clone();
                        val.cock.get_custom(state.as_str(), &i, &custom_option);
                        val.state = val.state.next();
                        s.set_user_data(val.clone());
                        s.pop_layer();
                        val.state.draw(s);
                    },
                ))
                .title("Input Custom Option"),
            );
        } else {
            let mut val = s.user_data::<UserData>().unwrap().clone();
            val.cock.from_str_part(state.as_str(), item);
            val.state = val.state.next();
            s.set_user_data(val.clone());
            s.pop_layer();
            val.state.draw(s);
        }
    });

    // `j` and `k` keys for navigation
    let select = OnEventView::new(select)
        .on_pre_event_inner('k', |s, _| {
            let cb = s.select_up(1);
            Some(EventResult::Consumed(Some(cb)))
        })
        .on_pre_event_inner('j', |s, _| {
            let cb = s.select_down(1);
            Some(EventResult::Consumed(Some(cb)))
        });

    siv.add_layer(
        Dialog::around(select.scrollable().fixed_size((40, 20)))
            .title(state.as_str())
            .button("Prev", |s| {
                let mut val = s.user_data::<UserData>().unwrap().clone();
                val.state = val.state.prev();
                s.set_user_data(val.clone());
                s.pop_layer();
                val.state.draw(s);
            }),
    );
}
