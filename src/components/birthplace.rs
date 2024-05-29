use super::*;

#[derive(Debug)]
enum Birthplace {
    Home,
    HomeOFF,
    HomeHOM,
    CCOW,
    BSOOO,
    Cave,
    Field,
    Forest,
    Temple,
    Battlefield,
    Alley,
    BTOI,
    CKTOP,
    SORH,
    APOADR,
    BOS,
    Prison,
    HSO,
    SL,
    Feywild,
    Shadowfell,
    APOEP,
    InnerPlane,
    OuterPlane,
}

impl Birthplace {
    fn as_str(&self) -> &'static str {
        match self {
            Birthplace::Home => "Home",
            Birthplace::HomeOFF => "Home of a family friend",
            Birthplace::HomeHOM => "Home of a healer or midwife",
            Birthplace::CCOW => "Carriage, cart, or wagon",
            Birthplace::BSOOO => "Barn, shed, or other outbuilding",
            Birthplace::Cave => "Cave",
            Birthplace::Field => "Field",
            Birthplace::Forest => "Forest",
            Birthplace::Temple => "Temple",
            Birthplace::Battlefield => "Battlefield",
            Birthplace::Alley => "Alley or street",
            Birthplace::BTOI => "Brothel, tavern, or inn",
            Birthplace::CKTOP => "Castle, keep, tower, or palace",
            Birthplace::SORH => "Sewer or rubbish heap",
            Birthplace::APOADR => "Among people of a different race",
            Birthplace::BOS => "On board a boat or a ship",
            Birthplace::Prison => "In a prison",
            Birthplace::HSO => "In the headquarters of a secret organization",
            Birthplace::SL => "In a sage's laboratory",
            Birthplace::Feywild => "In the Feywild",
            Birthplace::Shadowfell => "In the Shadowfell",
            Birthplace::APOEP => "On the Astral Plane or the Ethereal Plane",
            Birthplace::InnerPlane => "On an Inner Plane of your choice",
            Birthplace::OuterPlane => "On an Outer Plane of your choice",
        }
    }
}

#[component]
pub fn Birthplace() -> impl IntoView {
    let birthplace = match roll_die(100) {
        1..=50 => Birthplace::Home.as_str(),
        51..=55 => Birthplace::HomeOFF.as_str(),
        56..=63 => Birthplace::HomeHOM.as_str(),
        64..=65 => Birthplace::CCOW.as_str(),
        66..=68 => Birthplace::BSOOO.as_str(),
        69..=70 => Birthplace::Cave.as_str(),
        71..=72 => Birthplace::Field.as_str(),
        73..=74 => Birthplace::Forest.as_str(),
        75..=77 => Birthplace::Temple.as_str(),
        78 => Birthplace::Battlefield.as_str(),
        79..=80 => Birthplace::Alley.as_str(),
        81..=82 => Birthplace::BTOI.as_str(),
        83..=84 => Birthplace::CKTOP.as_str(),
        85 => Birthplace::SORH.as_str(),
        86..=88 => Birthplace::APOADR.as_str(),
        89..=91 => Birthplace::BOS.as_str(),
        92 => Birthplace::Prison.as_str(),
        93 => Birthplace::HSO.as_str(),
        94..=95 => Birthplace::SL.as_str(),
        96 => Birthplace::Feywild.as_str(),
        97 => Birthplace::Shadowfell.as_str(),
        98 => Birthplace::APOEP.as_str(),
        99 => Birthplace::InnerPlane.as_str(),
        100 => Birthplace::OuterPlane.as_str(),
        _ => unreachable!(),
    };

    view! {
        <div>
            <p>"Birthplace: "{birthplace}</p>
        </div>
    }
}
