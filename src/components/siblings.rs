use alignment::Alignment;
use relationship::Relationship;

use crate::names::{
    dwarves::{DWARVEN_FEMALE_NAMES, DWARVEN_MALE_NAMES},
    elves::{ELVEN_FEMALE_NAMES, ELVEN_MALE_NAMES},
    halfling::{HALFLING_FEMALE_NAMES, HALFLING_MALE_NAMES},
    human::{HUMAN_FEMALE_NAMES, HUMAN_MALE_NAMES},
    dragonborn::{DRAGONBORN_FEMALE_NAMES, DRAGONBORN_MALE_NAMES},
    gnome::{GNOME_FEMALE_NAMES, GNOME_MALE_NAMES},
};

use super::*;

#[component]
pub fn Siblings() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>()
        .expect("state to have been provided");

    let race = create_memo(move |_| state.get().race);

    let number_of_siblings = match roll_die(10) {
        1..=2 => 0,
        3..=4 => rand::thread_rng().gen_range(1..=3),
        5..=6 => rand::thread_rng().gen_range(1..=4) + 1,
        7..=8 => rand::thread_rng().gen_range(1..=6) + 2,
        9..=10 => rand::thread_rng().gen_range(1..=8) + 3,
        _ => unreachable!("number of siblings failed"),
    };

    let number_of_siblings = match number_of_siblings {
        0..=1 => {
            match race() {
                "Dwarf" => 0,
                "Elf" => 0,
                _ => number_of_siblings,
            }
        },
        _ => match race() {
            "Dwarf" => max(number_of_siblings - 2, 0),
            "Elf" => max(number_of_siblings - 2,0),
            _ => number_of_siblings,
        }
    };

    view! {
        <div>
            <p>"Number of siblings: " {number_of_siblings}</p>
            <For
                each=move || vec![0; number_of_siblings].into_iter().enumerate()
                key=|(_, state)|Uuid::new_v4()
                children=move |(index, _)| {
                    let gender = create_memo(move |_| {
                        match roll_die(2) {
                            1 => "Brother",
                            2 => "Sister",
                            _ => unreachable!("sibling gender failed"),
                        }
                    });

                    let name = sibling_name(gender(), race());

                    view! {
                        <hr />
                        <p>"Sibling: "{gender}</p>
                        <p>"Name: "{name}</p>
                        <Alignment />
                        <SiblingAge />
                        <Relationship />
                    }
                }
            />
        </div>
    }
}

pub fn sibling_name(gender: &str, race: &str) -> &'static str {
    match gender {
        "Brother" => {
            match race {
                "Dwarf" => DWARVEN_MALE_NAMES[(roll_die(102) - 1) as usize],
                "Elf" => ELVEN_MALE_NAMES[(roll_die(101) - 1) as usize],
                "Halfling" => HALFLING_MALE_NAMES[(roll_die(96) - 1) as usize],
                "Human" => HUMAN_MALE_NAMES[(roll_die(96) - 1) as usize],
                "Dragonborn" => DRAGONBORN_MALE_NAMES[(roll_die(101) - 1) as usize],
                "Gnome" => GNOME_MALE_NAMES[(roll_die(98) - 1) as usize],
                _ => "Jim",
            }
        },
        "Sister" => {
            match race {
                "Dwarf" => DWARVEN_FEMALE_NAMES[(roll_die(100) - 1) as usize],
                "Elf" => ELVEN_FEMALE_NAMES[(roll_die(159) - 1) as usize],
                "Halfling" => HALFLING_FEMALE_NAMES[(roll_die(94) - 1) as usize],
                "Human" => HUMAN_FEMALE_NAMES[(roll_die(95) - 1) as usize],
                "Dragonborn" => DRAGONBORN_FEMALE_NAMES[(roll_die(101) - 1) as usize],
                "Gnome" => GNOME_FEMALE_NAMES[(roll_die(95) - 1) as usize],
                _ => "Sue"
            }
        },
        _ => unreachable!{"sibling name gender failed"},
    }
}

#[component]
pub fn SiblingAge() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>()
        .expect("state to have been provided");

    let age = create_memo(move |_| state.get().age);

    let sibling_age = match roll_die(6) + roll_die(6) {
        2 => age(),
        3..=7 => age() + rand::thread_rng().gen_range(1..=10),
        8..=12 => age() - rand::thread_rng().gen_range(1..=10),
        _ => unreachable!("sibling age failed"),
    };

    let occupation = match sibling_age {
        n if n < 18 => "None",
        _ => match roll_die(100) {
            1..=5 => "Academic",
            6..=10 => match roll_die(12) {
                1 => "Barbarian",
                2 => "Bard",
                3 => "Cleric",
                4 => "Druid",
                5 => "Fighter",
                6 => "Monk",
                7 => "Paladin",
                8 => "Ranger",
                9 => "Rogue",
                10 => "Sorcerer",
                11 => "Warlock",
                12 => "Wizard",
                _ => unreachable!("sibling adventurer occupation failed"),
            },
            11 => "Aristocrat",
            12..=26 => "Artisan or guild member",
            27..=31 => "Criminal",
            32..=36 => "Entertainer",
            37..=38 => "Exile, hermit, or refugee",
            39..=43 => "Explorer or wanderer",
            44..=55 => "Farmer or herder",
            56..=60 => "Hunter or trapper",
            61..=75 => "Laborer",
            76..=80 => "Merchant",
            81..=85 => "Politician or bureaucrat",
            86..=90 => "Priest",
            91..=95 => "Sailor",
            96..=100 => "Soldier",
            _ => unreachable!("sibling occupation failed"),
        }
    };

    view! {
        <div>
            <p>"Age: " {sibling_age}</p>
            <p>"Occupation: " {occupation}</p>
        </div>
    }
}
