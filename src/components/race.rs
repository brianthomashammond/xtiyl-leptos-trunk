use crate::names::dwarves::{DWARVEN_FEMALE_NAMES, DWARVEN_MALE_NAMES, DWARVEN_SURNAMES};

use super::*;

#[derive(Debug)]
enum Race {
    Dwarf,
    Elf,
    Halfling,
    Human,
    Dragonborn,
    Gnome,
    HalfElf,
    HalfOrc,
    Tiefling,
}

impl Race {
    fn as_str(&self) -> &'static str {
        match self {
            Race::Dwarf => "Dwarf",
            Race::Elf => "Elf",
            Race::Halfling => "Halfling",
            Race::Human => "Human",
            Race::Dragonborn => "Dragonborn",
            Race::Gnome => "Gnome",
            Race::HalfElf => "Half-Elf",
            Race::HalfOrc => "Half-Orc",
            Race::Tiefling => "Tiefling",
        }
    }
}

#[component]
pub fn Race() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>()
        .expect("state to have been provided");

    let (race, set_race) = create_slice(
        state,
        |state| state.race,
        |state, n| state.race = n,
    );

    let gen_race = match roll_die(9) {
        1 => Race::Dwarf.as_str(),
        2 => Race::Elf.as_str(),
        3 => Race::Halfling.as_str(),
        4 => Race::Human.as_str(),
        5 => Race::Dragonborn.as_str(),
        6 => Race::Gnome.as_str(),
        7 => Race::HalfElf.as_str(),
        8 => Race::HalfOrc.as_str(),
        9 => Race::Tiefling.as_str(),
        _ => unreachable!("race failed"),
    };

    set_race(gen_race);

    let name = match roll_die(2) {
        1 => match gen_race {
            "Dwarf" => DWARVEN_MALE_NAMES[roll_die(102) as usize],
            _ => "Frank",
        },
        2 => match gen_race {
            "Dwarf" => DWARVEN_FEMALE_NAMES[roll_die(100) as usize],
            _ => "Miriam",
        }
        _ => unreachable!("player name gender failed")
    };

    let surname = match gen_race {
        "Dwarf" => DWARVEN_SURNAMES[roll_die(100) as usize],
        _ => "Rhoades"
    };

    view! {
        <div>
            <p>"Name: "{name}" "{surname}</p>
            <p>"Race: "{race}</p>
        </div>
    }
}
