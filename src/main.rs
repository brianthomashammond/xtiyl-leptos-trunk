use leptos::*;

use xtiyl::kernel::GlobalState;

use xtiyl::components::{
    alignment::Alignment,
    age::Age,
    background::Background,
    background_motivation::BackgroundMotivation,
    birthplace::Birthplace,
    class::Class,
    class_training::ClassTraining,
    family::Family,
    family_lifestyle::FamilyLifestyle,
    parent_knowledge::ParentKnowledge,
    race::Race,
    siblings::Siblings,
};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let state = create_rw_signal(GlobalState::default());
    provide_context(state);

    view! {
        <Class />
        <Race />
        <Background />
        <Alignment />
        <Age />
        <ClassTraining />
        <BackgroundMotivation />
        <hr />
        <Family />
        <Birthplace />
        <FamilyLifestyle />
        <hr />
        <ParentKnowledge />
        <hr />
        <Siblings />
    }
}
