use super::*;

#[component]
pub fn Relationship() -> impl IntoView {
    let relationship = match roll_die(4) + roll_die(4) + roll_die(4) {
        3..=4 => "Hostile",
        5..=10 => "Friendly",
        11..=12 => "Indifferent",
        _ => unreachable!("relationship failed",)
    };

    view! {
        <div>
            <p>"Relationship: " {relationship}</p>
        </div>
    }
}
