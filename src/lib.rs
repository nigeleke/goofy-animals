#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

use rand::Rng;

pub const DEFAULT_GOOFY_ANIMALS: GoofyAnimals<'static> = GoofyAnimals::new(
    &const_str::split!(include_str!("data/en_animals.txt"), "\n"),
    &const_str::split!(include_str!("data/en_adjectives.txt"), "\n"),
);

pub struct GoofyAnimals<'a> {
    animals: &'a [&'a str],
    adjectives: &'a [&'a str],
}

impl<'a> GoofyAnimals<'a> {
    pub const fn new(animals: &'a [&'a str], adjectives: &'a [&'a str]) -> Self {
        let total_animals = animals.len();
        let total_adjectives = adjectives.len();

        if total_animals < 1 {
            panic!("empty animals");
        }

        if total_adjectives < 2 {
            panic!("must have at least two adjectives");
        }

        if const_str::equal!(*animals.last().unwrap(), "") {
            panic!("trailing newline in animals");
        }

        if const_str::equal!(*adjectives.last().unwrap(), "") {
            panic!("trailing newline in adjectives");
        }

        Self {
            animals,
            adjectives,
        }
    }

    pub fn new_unchecked(animals: &'a [&'a str], adjectives: &'a [&'a str]) -> Self {
        Self {
            animals,
            adjectives,
        }
    }

    pub fn get_animals(&self) -> &'a [&'a str] {
        self.animals
    }

    pub fn get_adjectives(&self) -> &'a [&'a str] {
        self.adjectives
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(rng), level = tracing::Level::TRACE))]
    pub fn generate_name_parts(&self, rng: &mut impl Rng) -> (&'a str, &'a str, &'a str) {
        let (adjective_one, adjective_two) = loop {
            let one = rng.gen_range(0..self.adjectives.len());
            let two = rng.gen_range(0..self.adjectives.len());

            if one == two {
                continue;
            }

            break (one, two);
        };

        let animal = rng.gen_range(0..self.animals.len());

        #[cfg(feature = "tracing")]
        tracing::trace!(adjective_one, adjective_two, animal, "generated name");

        (
            self.adjectives[adjective_one],
            self.adjectives[adjective_two],
            self.animals[animal],
        )
    }

    #[inline]
    #[cfg(feature = "alloc")]
    pub fn generate_name(&self, rng: &mut impl Rng) -> ::alloc::string::String {
        let (adjective_one, adjective_two, animal) = self.generate_name_parts(rng);

        ::alloc::format!("{adjective_one}-{adjective_two}-{animal}")
    }
}

#[inline]
pub fn generate_name_parts(rng: &mut impl Rng) -> (&'static str, &'static str, &'static str) {
    DEFAULT_GOOFY_ANIMALS.generate_name_parts(rng)
}

#[inline]
#[cfg(feature = "alloc")]
pub fn generate_name(rng: &mut impl Rng) -> ::alloc::string::String {
    DEFAULT_GOOFY_ANIMALS.generate_name(rng)
}

#[cfg(test)]
mod test {
    use super::DEFAULT_GOOFY_ANIMALS;

    use pretty_assertions::assert_eq;

    #[test]
    fn animals() {
        assert_eq!(DEFAULT_GOOFY_ANIMALS.get_animals().len(), 355);
    }

    #[test]
    fn adjectives() {
        assert_eq!(DEFAULT_GOOFY_ANIMALS.get_adjectives().len(), 1300);
    }

    #[test]
    #[cfg_attr(feature = "tracing", tracing_test::traced_test)]
    fn name_generation() {
        use rand::SeedableRng;
        use rand_chacha::ChaCha20Rng;

        let mut rng = ChaCha20Rng::seed_from_u64(0x1337);

        assert_eq!(
            DEFAULT_GOOFY_ANIMALS.generate_name_parts(&mut rng),
            ("healthy", "frivolous", "dove"),
        );
        assert_eq!(
            DEFAULT_GOOFY_ANIMALS.generate_name_parts(&mut rng),
            ("glorious", "meager", "polar bear"),
        );
        assert_eq!(
            DEFAULT_GOOFY_ANIMALS.generate_name_parts(&mut rng),
            ("thankful", "elastic", "clownfish"),
        );
        assert_eq!(
            DEFAULT_GOOFY_ANIMALS.generate_name_parts(&mut rng),
            ("vigilant", "troubled", "firefly"),
        );
        assert_eq!(
            DEFAULT_GOOFY_ANIMALS.generate_name_parts(&mut rng),
            ("handsome", "modest", "porcupine"),
        );
        assert_eq!(
            DEFAULT_GOOFY_ANIMALS.generate_name_parts(&mut rng),
            ("sunny", "wonderful", "dormouse"),
        );
        assert_eq!(
            DEFAULT_GOOFY_ANIMALS.generate_name_parts(&mut rng),
            ("treasured", "woozy", "deer"),
        );
        assert_eq!(
            DEFAULT_GOOFY_ANIMALS.generate_name_parts(&mut rng),
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

        let mut rng = ChaCha20Rng::seed_from_u64(0x1337);

        assert_eq!(
            DEFAULT_GOOFY_ANIMALS.generate_name(&mut rng),
            "healthy-frivolous-dove",
        );
    }
}
