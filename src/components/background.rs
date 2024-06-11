use super::*;

#[derive(Debug)]
enum Background {
    Acolyte,
    Charlatan,
    Criminal,
    Entertainer,
    FolkHero,
    GuildArtisan,
    Hermit,
    Noble,
    Outlander,
    Sage,
    Sailor,
    Soldier,
    Urchin,
}

impl Background {
    fn as_str(&self) -> &'static str {
        match self {
            Background::Acolyte => "Acolyte",
            Background::Charlatan => "Charlatan",
            Background::Criminal => "Criminal",
            Background::Entertainer => "Entertainer",
            Background::FolkHero => "Folk Hero",
            Background::GuildArtisan => "Guild Artisan",
            Background::Hermit => "Hermit",
            Background::Noble => "Noble",
            Background::Outlander => "Outlander",
            Background::Sage => "Sage",
            Background::Sailor => "Sailor",
            Background::Soldier => "Soldier",
            Background::Urchin => "Urchin",
        }
    }
}

#[component]
pub fn Background() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>()
        .expect("state to have been provided");

    let (background, set_background) = create_slice(
        state,
        |state| state.background,
        |state, n| state.background = n,
    );

    let gen_background = match roll_die(13) {
        1 => Background::Acolyte.as_str(),
        2 => Background::Charlatan.as_str(),
        3 => Background::Criminal.as_str(),
        4 => Background::Entertainer.as_str(),
        5 => Background::FolkHero.as_str(),
        6 => Background::GuildArtisan.as_str(),
        7 => Background::Hermit.as_str(),
        8 => Background::Noble.as_str(),
        9 => Background::Outlander.as_str(),
        10 => Background::Sage.as_str(),
        11 => Background::Sailor.as_str(),
        12 => Background::Soldier.as_str(),
        13 => Background::Urchin.as_str(),
        _ => unreachable!("background failed"),
    };

    set_background(gen_background);

    view! {
        <div>
            <p>"Background: "{background}</p>
        </div>
    }
}
