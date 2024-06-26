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
        _ => unreachable!("family lifestyle failed"),
    };

    let lifestyle_modifier = match lifestyle{
        "Wretched" => -40,
        "Squalid" => -20,
        "Poor" => -10,
        "Modest" => 0,
        "Comfortable" => 10,
        "Wealthy" => 20,
        "Aristocratic" => 40,
        _ => unreachable!("lifestyle modifier failed"),
    };

    let childhood_home = match roll_die(100) + lifestyle_modifier {
        i32::MIN..=0 => "On the streets",
        1..=20 => "Rundown shack",
        21..=30 => "No permanent residence; you moved around a lot",
        31..=40 => "Encampment or village in the wilderness",
        41..=50 => "Apartment in a rundown neighborhood",
        51..=70 => "Small house",
        71..=90 => "Large house",
        91..=110 => "Mansion",
        111..=i32::MAX => "Palace or castle",
        _ => unreachable!("childhood home failed"),
    };



    view! {
        <div>
            <p>"Family Lifestyle: "{lifestyle}</p>
            <p>"Childhood Home: "{childhood_home}</p>
        </div>
    }
}
