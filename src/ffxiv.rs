use anyhow::Context;
use ironworks::file::{File, cmp};
use std::fs;

#[derive(Copy, Clone, Debug)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub fn to_str(&self) -> &'static str {
        match self {
            Gender::Male => "Male",
            Gender::Female => "Female",
        }
    }
}

#[derive(Debug)]
pub struct TribalColourParams {
    pub race: String,
    pub tribe: String,
    pub gender: Gender,
    pub skin_colours: [cmp::Color; 192],
    pub hair_colours: [cmp::Color; 192],
}

pub const TRIBES: [(&str, &str); 16] = [
    ("Hyur", "Midlander"),
    ("Hyur", "Highlander"),
    ("Elezen", "Wildwood"),
    ("Elezen", "Duskwight"),
    ("Lalafell", "Plainsfolk"),
    ("Lalafell", "Dunesfolk"),
    ("Miqo'te", "Seeker of the Sun"),
    ("Miqo'te", "Keeper of the Moon"),
    ("Roegadyn", "Sea Wolf"),
    ("Roegadyn", "Hellsguard"),
    ("Au Ra", "Raen"),
    ("Au Ra", "Xaela"),
    ("Hrothgar", "Helions"),
    ("Hrothgar", "The Lost"),
    ("Viera", "Rava"),
    ("Viera", "Veena"),
];

pub fn load_racial_colours(cmp_path: &str) -> anyhow::Result<Vec<TribalColourParams>> {
    println!("Reading from: {cmp_path}");
    let cmp_f = fs::File::open(cmp_path).context("Error opening human.cmp file")?;
    let cmp_data =
        cmp::CharacterMakeParameters::read(cmp_f).context("Error reading human.cmp file")?;

    let res_len = TRIBES.len() * 2; // *2 for male and female
    let mut res = Vec::with_capacity(res_len);
    let iter = TRIBES
        .iter()
        .flat_map(|r| std::iter::repeat_n(r, 2))
        .zip(cmp_data.races())
        .zip([Gender::Male, Gender::Female].iter().cycle());
    for ((&(race, tribe), cmp_params), &gender) in iter {
        let tribe_cmp = TribalColourParams {
            race: race.to_string(),
            tribe: tribe.to_string(),
            gender,
            skin_colours: cmp_params.skin_interface()[..192].try_into()?,
            hair_colours: cmp_params.hair_interface()[..192].try_into()?,
        };
        res.push(tribe_cmp);
    }

    println!("Read {res_len} tribe colour parameter sets");
    Ok(res)
}
