pub mod age;
pub mod alignment;
pub mod background;
pub mod background_motivation;
pub mod birthplace;
pub mod class;
pub mod class_training;
pub mod family;
pub mod family_lifestyle;
pub mod parent_knowledge;
pub mod race;
pub mod siblings;

use leptos::*;
use rand::Rng;
use std::cmp::max;
use uuid::Uuid;

use crate::kernel::{ GlobalState, roll_die };
