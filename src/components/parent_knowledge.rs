use alignment::Alignment;
use relationship::Relationship;

use super::*;

#[derive(Debug)]
pub enum ParentKnowledge {
    Known,
    Unknown,
}

impl ParentKnowledge {
    fn as_str(&self) -> &'static str {
        match self {
            ParentKnowledge::Known => "You know who your parents are or were.",
            ParentKnowledge::Unknown => "You do not know who your parents are or were.",
        }
    }
}

#[component]
pub fn ParentKnowledge() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>()
        .expect("state to have been provided");

    let (parent_knowledge, set_parent_knowledge) = create_slice(
        state,
        |state| state.parent_knowledge,
        |state, n| state.parent_knowledge = n,
    );

    let race = create_memo(
        move |_| state.get().race
    );

    let parent_knowledge_text = match roll_die(100) {
        1..=95 => {
            set_parent_knowledge(true);

            ParentKnowledge::Known.as_str()
        },
        96..=100 => {
            set_parent_knowledge(false);

            ParentKnowledge::Unknown.as_str()
        },
        _ => unreachable!("parent knowledge failed"),
    };


    let (race_mom, race_dad) = match race() {
        "Half-Elf" => {
            match roll_die(8) {
                1..=5 => {
                    match roll_die(2) {
                        1 => ("Elf", "Human"),
                        2 => ("Human", "Elf"),
                        _ => unreachable!("half-elf parent race 1 failed"),
                    }
                },
                6 => {
                    match roll_die(2) {
                        1 => ("Elf", "Half-Elf"),
                        2 => ("Half-Elf", "Elf"),
                        _ => unreachable!("half-elf parent race 2 failed"),
                    } 
                },
                7 => {
                    match roll_die(2) {
                        1 => ("Half-Elf", "Human"),
                        2 => ("Human", "Half-Elf"),
                        _ => unreachable!("half-elf parent race 3 failed"),
                    } 
                },
                8 => {
                    match roll_die(2) {
                        1 => ("Half-Elf", "Half-Elf"),
                        2 => ("Half-Elf", "Half-Elf"),
                        _ => unreachable!("half-elf parent race 4 failed"),
                    } 
                },
                _ => unreachable!("half-elf parent race 5 failed"),
            }
        },
        "Half-Orc" => {
            match roll_die(8) {
                1..=5 => {
                    match roll_die(2) {
                        1 => ("Orc", "Human"),
                        2 => ("Human", "Orc"),
                        _ => unreachable!("half-orc parent race 1 failed"),
                    }
                },
                6 => {
                    match roll_die(2) {
                        1 => ("Orc", "Half-Orc"),
                        2 => ("Half-Orc", "Orc"),
                        _ => unreachable!("half-orc parent race 2 failed"),
                    } 
                },
                7 => {
                    match roll_die(2) {
                        1 => ("Half-Orc", "Human"),
                        2 => ("Human", "Half-Orc"),
                        _ => unreachable!("half-orc parent race 3 failed"),
                    } 
                },
                8 => {
                    match roll_die(2) {
                        1 => ("Half-Orc", "Half-Orc"),
                        2 => ("Half-Orc", "Half-Orc"),
                        _ => unreachable!("half-orc parent race 4 failed"),
                    } 
                },
                _ => unreachable!("half-orc parent race 5 failed"),
            }
        },
        "Tiefling" => {
            match roll_die(8) {
                1..=4 => ("Human, dormant Infernal lineage", "Human, dormant Infernal lineage"),
                5..=6 => {
                    match roll_die(2) {
                        1 => ("Tiefling", "Human"),
                        2 => ("Human", "Tiefling"),
                        _ => unreachable!("tiefling parent race 1 failed"),
                    } 
                },
                7 => {
                    match roll_die(2) {
                        1 => ("Tiefling", "Devil"),
                        2 => ("Devil", "Tiefling"),
                        _ => unreachable!("tiefling parent race 2 failed"),
                    } 
                },
                8 => {
                    match roll_die(2) {
                        1 => ("Human", "Devil"),
                        2 => ("Devil", "Human"),
                        _ => unreachable!("tiefling parent race 3 failed"),
                    } 
                },
                _ => unreachable!("tiefling parent race 4 failed"),
            }
        },
        _ => (race(), race()),
    };

    view! {
        <div>
            <p>"Parent Knowledge: "{parent_knowledge_text}</p>
            <Show
                when=move || { parent_knowledge() }
            >
                <p>"Mother's Race: "{race_mom}</p>
                <Alignment />
                <ParentAge />
                <Relationship />
                <hr />
                <p>"Father's Race: "{race_dad}</p>
                <Alignment />
                <ParentAge />
                <Relationship />
            </Show>
        </div>
    }
}

#[component]
pub fn ParentAge() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>()
        .expect("state to have been provided");

    let age = create_memo(move |_| state.get().age);

    let parent_age = age() + 18 + rand::thread_rng().gen_range(0..=30);

    let occupation =  match roll_die(100) {
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
    };

    view! {
        <div>
            <p>"Age: " {parent_age}</p>
            <p>"Occupation: " {occupation}</p>
        </div>
    }
}
