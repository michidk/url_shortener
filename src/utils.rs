use rand::seq::SliceRandom;

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
