use crate::ffxiv::TribalColourParams;
use ironworks::file::cmp;
use serde::Serialize;
use tera::{Context, Tera};

const COLOUR_COLUMNS: usize = 8;

#[derive(Serialize)]
struct Race {
    id: usize,
    name: String,
    tribes: Vec<Tribe>,
}

#[derive(Serialize)]
struct Tribe {
    id: usize,
    name: String,
    gender: &'static str,
    palettes: [Palette; 2],
}

#[derive(Serialize)]
struct Palette {
    name: &'static str,
    rows: Vec<Vec<[u8; 3]>>,
}

fn palette(name: &'static str, colours: &[cmp::Color; 192]) -> Palette {
    Palette {
        name,
        rows: colours
            .chunks(COLOUR_COLUMNS)
            .map(|row| row.iter().map(|c| [c.red(), c.green(), c.blue()]).collect())
            .collect(),
    }
}

pub fn render_index(cmps: &[TribalColourParams]) -> anyhow::Result<String> {
    let mut races_outp: Vec<Race> = Vec::new();

    // Taking advantage of the fact that the colours slice is grouped by race and tribe
    let (races, _) = cmps.as_chunks::<4>();
    for (race_id, race) in races.iter().enumerate() {
        let mut race_d = Race {
            id: race_id,
            name: race[0].race.clone(),
            tribes: Vec::with_capacity(2),
        };
        let (tribes, _) = race.as_chunks::<2>();
        for (tribe_id, tribe) in tribes.iter().flatten().enumerate() {
            let tribe_d = Tribe {
                id: tribe_id,
                name: tribe.tribe.clone(),
                gender: tribe.gender.to_str(),
                palettes: [
                    palette("Skin", &tribe.skin_colours),
                    palette("Hair", &tribe.hair_colours),
                ],
            };
            race_d.tribes.push(tribe_d);
        }
        races_outp.push(race_d);
    }

    let mut tera = Tera::default();
    tera.add_raw_template("index.html", include_str!("../templates/index.html"))?;
    let mut context = Context::new();
    context.insert("races", &races_outp);
    Ok(tera.render("index.html", &context)?)
}
