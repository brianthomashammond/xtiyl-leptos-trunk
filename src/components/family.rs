use super::*;

#[derive(Debug)]
enum Family {
    None,
    Institution,
    Temple,
    Orphanage,
    Guardian,
    Tribe,
    Grandparents,
    Adoptive,
    SingleFather,
    SingleMother,
    MotherFather,
}

impl Family {
    fn as_str(&self) -> &'static str {
        match self {
            Family::None => "None",
            Family::Institution => "Institution, such as an asylum",
            Family::Temple => "Temple",
            Family::Orphanage => "Orphanage",
            Family::Guardian => "Guardian",
            Family::Tribe => "Paternal or maternal aunt, uncle, or both; or extended family such as a tribe or clan",
            Family::Grandparents => "Paternal or maternal grandparent(s)",
            Family::Adoptive => "Adoptive family (same or different race)",
            Family::SingleFather => "Single father or stepfather",
            Family::SingleMother => "Single mother or stepmother",
            Family::MotherFather => "Mother and father",
        }
    }
}

#[component]
pub fn Family() -> impl IntoView {
    let family = match roll_die(100) {
        1 => Family::None.as_str(),
        2 => Family::Institution.as_str(),
        3 => Family::Temple.as_str(),
        4..=5 => Family::Orphanage.as_str(),
        6..=7 => Family::Guardian.as_str(),
        8..=15 => Family::Tribe.as_str(),
        16..=25 => Family::Grandparents.as_str(),
        26..=35 => Family::Adoptive.as_str(),
        36..=55 => Family::SingleFather.as_str(),
        56..=75 => Family::SingleMother.as_str(),
        76..=100 => Family::MotherFather.as_str(),
        _ => unreachable!(),
    };

    view! {
        <div>
            <p>"Family: " {family}</p>
        </div>
    }
}
