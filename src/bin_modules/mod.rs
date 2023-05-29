use crate::{
    cock_parts::{
    Abnormalities, Aesthetic,
    Balls, Circumcision,
    Curvature, Shape,
    SizeType, Veininess,
    },
    GetVariants,
    ID,
    CockStruct
};

pub mod standard_prompt;
pub mod tui_prompt;

#[derive(Clone)]
struct UserData {
    user: ID,
    cock: CockStruct,
    state: AppState,
}

impl UserData {
    #[allow(dead_code)]
    fn default() -> UserData {
        UserData {
            user: ID::Anonymous,
            cock: CockStruct::default(),
            state: AppState::default(),
        }
    }
}

#[derive(Clone, Copy)]
enum AppState {
    Home,
    Id,
    Abnormalities,
    Aesthetic,
    Balls,
    Circumcision,
    Curvature,
    Shape,
    Size,
    Veininess,
    Result,
}

impl AppState {
    fn default() -> AppState {
        AppState::Home
    }

    fn options(&self) -> Vec<String> {
        match self {
            AppState::Abnormalities => Abnormalities::get_variants(),
            AppState::Aesthetic => Aesthetic::get_variants(),
            AppState::Balls => Balls::get_variants(),
            AppState::Circumcision => Circumcision::get_variants(),
            AppState::Curvature => Curvature::get_variants(),
            AppState::Id => ID::get_variants(),
            AppState::Shape => Shape::get_variants(),
            AppState::Size => SizeType::get_variants(),
            AppState::Veininess => Veininess::get_variants(),
            _ => Vec::default(),
        }
    }

    fn as_str(&self) -> &str {
        match self {
            AppState::Home => "Home",
            AppState::Id => "ID",
            AppState::Abnormalities => "Abnormalities",
            AppState::Aesthetic => "Aesthetic",
            AppState::Balls => "Balls",
            AppState::Circumcision => "Circumcision",
            AppState::Curvature => "Curvature",
            AppState::Shape => "Shape",
            AppState::Size => "Size",
            AppState::Veininess => "Veininess",
            AppState::Result => "Result",
        }
    }

    fn next(&self) -> AppState {
        match self {
            AppState::Home => AppState::Id,
            AppState::Id => AppState::Size,
            AppState::Size => AppState::Abnormalities,
            AppState::Abnormalities => AppState::Aesthetic,
            AppState::Aesthetic => AppState::Balls,
            AppState::Balls => AppState::Circumcision,
            AppState::Circumcision => AppState::Curvature,
            AppState::Curvature => AppState::Shape,
            AppState::Shape => AppState::Veininess,
            AppState::Veininess => AppState::Result,
            AppState::Result => AppState::Result,
        }
    }

    fn prev(&self) -> AppState {
        match self {
            AppState::Home => AppState::Home,
            AppState::Id => AppState::Home,
            AppState::Size => AppState::Id,
            AppState::Abnormalities => AppState::Size,
            AppState::Aesthetic => AppState::Abnormalities,
            AppState::Balls => AppState::Aesthetic,
            AppState::Circumcision => AppState::Balls,
            AppState::Curvature => AppState::Circumcision,
            AppState::Shape => AppState::Curvature,
            AppState::Veininess => AppState::Shape,
            AppState::Result => AppState::Veininess,
        }
    }
}
