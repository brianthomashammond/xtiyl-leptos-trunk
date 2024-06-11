use super::*;

#[component]
pub fn Age() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>()
        .expect("state to have been provided");

    let (age, set_age) = create_slice(
        state,
        |state| state.age,
        |state, n| state.age = n,
    );

    let gen_age = match roll_die(100) {
        1..=20 => match roll_die(3) {
            1 => 18,
            2 => 19,
            3 => 20,
            _ => unreachable!("age 1 failed"),
        },
        21..=59 => 20 + roll_die(10),
        60..=69 => 30 + roll_die(10),
        70..=89 => 40 + roll_die(10),
        90..=99 => 50 + roll_die(10),
        100 => 60 + roll_die(20),
        _ => unreachable!("age 2 failed"),
    };

    set_age(gen_age);

    view! {
        <div>
            <p>"Age: "{age}</p>
        </div>
    }
}
