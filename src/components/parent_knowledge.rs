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
        _ => unreachable!(),
    };


    let (race_mom, race_dad) = match race() {
        "Half-Elf" => {
            match roll_die(8) {
                1..=5 => {
                    match roll_die(2) {
                        1 => ("Elf", "Human"),
                        2 => ("Human", "Elf"),
                        _ => unreachable!(),
                    }
                },
                6 => {
                    match roll_die(2) {
                        1 => ("Elf", "Half-Elf"),
                        2 => ("Half-Elf", "Elf"),
                        _ => unreachable!(),
                    } 
                },
                7 => {
                    match roll_die(2) {
                        1 => ("Half-Elf", "Human"),
                        2 => ("Human", "Half-Elf"),
                        _ => unreachable!(),
                    } 
                },
                8 => {
                    match roll_die(2) {
                        1 => ("Half-Elf", "Half-Elf"),
                        2 => ("Half-Elf", "Half-Elf"),
                        _ => unreachable!(),
                    } 
                },
                _ => unreachable!(),
            }
        },
        "Half-Orc" => {
            match roll_die(8) {
                1..=5 => {
                    match roll_die(2) {
                        1 => ("Orc", "Human"),
                        2 => ("Human", "Orc"),
                        _ => unreachable!(),
                    }
                },
                6 => {
                    match roll_die(2) {
                        1 => ("Orc", "Half-Orc"),
                        2 => ("Half-Orc", "Orc"),
                        _ => unreachable!(),
                    } 
                },
                7 => {
                    match roll_die(2) {
                        1 => ("Half-Orc", "Human"),
                        2 => ("Human", "Half-Orc"),
                        _ => unreachable!(),
                    } 
                },
                8 => {
                    match roll_die(2) {
                        1 => ("Half-Orc", "Half-Orc"),
                        2 => ("Half-Orc", "Half-Orc"),
                        _ => unreachable!(),
                    } 
                },
                _ => unreachable!(),
            }
        },
        "Tiefling" => {
            match roll_die(8) {
                1..=4 => ("Human, dormant Infernal lineage", "Human, dormant Infernal lineage"),
                5..=6 => {
                    match roll_die(2) {
                        1 => ("Tiefling", "Human"),
                        2 => ("Human", "Tiefling"),
                        _ => unreachable!(),
                    } 
                },
                7 => {
                    match roll_die(2) {
                        1 => ("Tiefling", "Devil"),
                        2 => ("Devil", "Tiefling"),
                        _ => unreachable!(),
                    } 
                },
                8 => {
                    match roll_die(2) {
                        1 => ("Human", "Devil"),
                        2 => ("Devil", "Human"),
                        _ => unreachable!(),
                    } 
                },
                _ => unreachable!(),
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
                <p>"Father's Race: "{race_dad}</p>
            </Show>
        </div>
    }
}
