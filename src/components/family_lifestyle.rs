use super::*;

#[derive(Debug)]
enum FamilyLifestyle {
    Wretched,
    Squalid,
    Poor,
    Modest,
    Comfortable,
    Wealthy,
    Aristocratic,
}

impl FamilyLifestyle {
    fn as_str(&self) -> &'static str {
        match self {
            FamilyLifestyle::Wretched => "Wretched",
            FamilyLifestyle::Squalid => "Squalid",
            FamilyLifestyle::Poor => "Poor",
            FamilyLifestyle::Modest => "Modest",
            FamilyLifestyle::Comfortable => "Comfortable",
            FamilyLifestyle::Wealthy => "Wealthy",
            FamilyLifestyle::Aristocratic => "Aristocratic",
        }
    }
}

#[component]
pub fn FamilyLifestyle() -> impl IntoView {
    let lifestyle = match roll_die(6) + roll_die(6) + roll_die(6) {
        3 => FamilyLifestyle::Wretched.as_str(),
        4..=5 => FamilyLifestyle::Squalid.as_str(),
        6..=8 => FamilyLifestyle::Poor.as_str(),
        9..=12 => FamilyLifestyle::Modest.as_str(),
        13..=15 => FamilyLifestyle::Comfortable.as_str(),
        16..=17 => FamilyLifestyle::Wealthy.as_str(),
        18 => FamilyLifestyle::Aristocratic.as_str(),
        _ => unreachable!(),
    };

    view! {
        <div>
            <p>"Family Lifestyle: "{lifestyle}</p>
        </div>
    }
}
