use rand::Rng;

#[derive(Copy, Clone, Debug, Default)]
pub struct GlobalState {
    pub age: i32,
    pub background: &'static str,
    pub class: &'static str,
    pub family_lifestyle: &'static str,
    pub parent_knowledge: bool,
    pub race: &'static str,
}

pub fn roll_die(size: i32) -> i32 {
    rand::thread_rng().gen_range(1..=size)
}
