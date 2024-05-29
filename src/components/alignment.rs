use super::*;

#[derive(Debug)]
enum Alignment {
    LawfulGood,
    LawfulNeutral,
    LawfulEvil,
    NeutralGood,
    Neutral,
    NeutralEvil,
    ChaoticGood,
    ChaoticNeutral,
    ChaoticEvil,
}

impl Alignment {
    fn as_str(&self) -> &'static str {
        match self {
            Alignment::LawfulGood => "Lawful Good",
            Alignment::LawfulNeutral => "Lawful Neutral",
            Alignment::LawfulEvil => "Lawful Evil",
            Alignment::NeutralGood => "Neutral Good",
            Alignment::Neutral => "Neutral",
            Alignment::NeutralEvil => "Neutral Evil",
            Alignment::ChaoticGood => "Chaotic Good",
            Alignment::ChaoticNeutral => "Chaotic Neutral",
            Alignment::ChaoticEvil => "Chaotic Evil",
        }
    }
}

#[component]
pub fn Alignment() -> impl IntoView {
    let alignment = match roll_die(6) + roll_die(6) + roll_die(6) {
        3 => match roll_die(2) {
            1 => Alignment::ChaoticEvil.as_str(), 
            2 => Alignment::ChaoticNeutral.as_str(),
            _ => unreachable!(),
        },
        4..=5 => Alignment::LawfulEvil.as_str(),
        6..=8 => Alignment::NeutralEvil.as_str(),
        9..=12 => Alignment::Neutral.as_str(),
        13..=15 => Alignment::NeutralGood.as_str(),
        16 => Alignment::LawfulGood.as_str(),
        17 => Alignment::LawfulNeutral.as_str(),
        18 => match roll_die(2) {
            1 => Alignment::ChaoticGood.as_str(), 
            2 => Alignment::ChaoticNeutral.as_str(),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };

    view! {
        <div>
            <p>"Alignment: "{alignment}</p>
        </div>
    }
}
