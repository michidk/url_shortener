use crate::{database::Database, AppError};
use rand::{distributions::Alphanumeric, seq::SliceRandom, thread_rng, Rng};
use rocket::State;

pub(crate) fn random_string(n: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}

pub(crate) fn gen_rnd_id(db: State<Database>) -> Result<String, AppError> {
    // TODO: add max retries
    loop {
        let s = random_string(6);
        if !db.urls.contains_key(s.as_bytes())? {
            return Ok(s);
        }
    }
}

pub(crate) fn get_rnd_spell() -> String {
    let slugs = vec![
        "As simple as it gets",
        "Dead simple",
        "Simple setup, short links",
        "✂️✂️✂️",
    ];
    let selection = slugs.choose(&mut rand::thread_rng());
    String::from(*selection.unwrap())
}
