use super::*;

#[derive(Debug)]
enum AdventuringClass {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

impl AdventuringClass {
    fn as_str(&self) -> &'static str {
        match self {
            AdventuringClass::Barbarian => "Barbarian",
            AdventuringClass::Bard => "Bard",
            AdventuringClass::Cleric => "Cleric",
            AdventuringClass::Druid => "Druid",
            AdventuringClass::Fighter => "Fighter",
            AdventuringClass::Monk => "Monk",
            AdventuringClass::Paladin => "Paladin",
            AdventuringClass::Ranger => "Ranger",
            AdventuringClass::Rogue => "Rogue",
            AdventuringClass::Sorcerer => "Sorcerer",
            AdventuringClass::Warlock => "Warlock",
            AdventuringClass::Wizard => "Wizard",
        }
    }
}

#[component]
pub fn Class() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>()
        .expect("state to have been provided");

    let (class, set_class) = create_slice(
        state,
        |state| state.class,
        |state, n| state.class = n,
    );

    let gen_class = match roll_die(12) {
        1 => AdventuringClass::Barbarian.as_str(),
        2 => AdventuringClass::Bard.as_str(),
        3 => AdventuringClass::Cleric.as_str(),
        4 => AdventuringClass::Druid.as_str(),
        5 => AdventuringClass::Fighter.as_str(),
        6 => AdventuringClass::Monk.as_str(),
        7 => AdventuringClass::Paladin.as_str(),
        8 => AdventuringClass::Ranger.as_str(),
        9 => AdventuringClass::Rogue.as_str(),
        10 => AdventuringClass::Sorcerer.as_str(),
        11 => AdventuringClass::Warlock.as_str(),
        12 => AdventuringClass::Wizard.as_str(),
        _ => unreachable!("adventuring class failed"),
    };

    set_class(gen_class);

    view! {
        <div>
            <p>"Class: "{class}</p>
        </div>
    }
}
