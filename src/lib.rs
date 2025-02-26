#[cfg(feature = "alloc")]
extern crate alloc;

use rand::Rng;

const ANIMALS: &[&str] = &const_str::split!(include_str!("data/en_animals.txt"), "\n");
const TOTAL_ANIMALS: usize = ANIMALS.len();
const ADJECTIVES: &[&str] = &const_str::split!(include_str!("data/en_adjectives.txt"), "\n");
const TOTAL_ADJECTIVES: usize = ADJECTIVES.len();

const _: () = {
    if const_str::equal!(*ANIMALS.last().unwrap(), "") {
        panic!("trailing newline in animals");
    }

    if const_str::equal!(*ADJECTIVES.last().unwrap(), "") {
        panic!("trailing newline in adjectives");
    }
};

#[cfg_attr(feature = "tracing", tracing::instrument(skip(rng), level = tracing::Level::TRACE))]
pub fn generate_name_parts(rng: &mut impl Rng) -> (&str, &str, &str) {
    let (adjective_one, adjective_two) = loop {
        let one = rng.gen_range(0..TOTAL_ADJECTIVES);
        let two = rng.gen_range(0..TOTAL_ADJECTIVES);

        if one == two {
            continue;
        }

        break (one, two);
    };

    let animal = rng.gen_range(0..TOTAL_ANIMALS);

    #[cfg(feature = "tracing")]
    tracing::trace!(adjective_one, adjective_two, animal, "generated name");

    (
        ADJECTIVES[adjective_one],
        ADJECTIVES[adjective_two],
        ANIMALS[animal],
    )
}

#[inline]
#[cfg(feature = "alloc")]
pub fn generate_name(rng: &mut impl Rng) -> ::alloc::string::String {
    let (adjective_one, adjective_two, animal) = generate_name_parts(rng);

    ::alloc::format!("{adjective_one}-{adjective_two}-{animal}")
}

#[cfg(test)]
mod test {
    use super::{TOTAL_ADJECTIVES, TOTAL_ANIMALS};
    use pretty_assertions::assert_eq;

    #[test]
    fn animals() {
        assert_eq!(TOTAL_ANIMALS, 355);
    }

    #[test]
    fn adjectives() {
        assert_eq!(TOTAL_ADJECTIVES, 1300);
    }

    #[test]
    #[cfg_attr(feature = "tracing", tracing_test::traced_test)]
    fn name_generation() {
        use rand::SeedableRng;
        use rand_chacha::ChaCha20Rng;

        use super::generate_name_parts;

        let mut rng = ChaCha20Rng::seed_from_u64(0x1337);

        assert_eq!(
            generate_name_parts(&mut rng),
            ("healthy", "frivolous", "dove"),
        );
        assert_eq!(
            generate_name_parts(&mut rng),
            ("glorious", "meager", "polar bear"),
        );
        assert_eq!(
            generate_name_parts(&mut rng),
            ("thankful", "elastic", "clownfish"),
        );
        assert_eq!(
            generate_name_parts(&mut rng),
            ("vigilant", "troubled", "firefly"),
        );
        assert_eq!(
            generate_name_parts(&mut rng),
            ("handsome", "modest", "porcupine"),
        );
        assert_eq!(
            generate_name_parts(&mut rng),
            ("sunny", "wonderful", "dormouse"),
        );
        assert_eq!(
            generate_name_parts(&mut rng),
            ("treasured", "woozy", "deer"),
        );
        assert_eq!(
            generate_name_parts(&mut rng),
            ("mealy", "cylindrical", "dog"),
        );

        #[cfg(all(feature = "tracing", feature = "alloc"))]
        {
            logs_assert(|lines: &[&str]| {
                const EXPECTED: usize = 8;

                match lines
                    .iter()
                    .filter(|line| line.contains("generated name"))
                    .count()
                {
                    EXPECTED => Ok(()),
                    n => Err(::alloc::format!("Expected {EXPECTED} logs, but got {n}")),
                }
            });
        }
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn name_generation_alloc() {
        use rand::SeedableRng;
        use rand_chacha::ChaCha20Rng;

        use super::generate_name;

        let mut rng = ChaCha20Rng::seed_from_u64(0x1337);

        assert_eq!(generate_name(&mut rng), "healthy-frivolous-dove",);
    }
}
