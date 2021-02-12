use rand::seq::SliceRandom;

mod syllables;
use syllables::SYLLABLES;

fn choose_random_syllable() -> String {
    (*SYLLABLES
        .choose(&mut rand::thread_rng())
        .expect("Empty syllable list"))
    .to_owned()
}

pub fn make_name(max_len: usize) -> String {
    let mut name = String::with_capacity(max_len);
    let mut current_len = 0usize;

    loop {
        let next_syllable = choose_random_syllable();
        let next_syllable_len = next_syllable.len();
        if current_len + next_syllable_len <= max_len {
            name += &next_syllable;
            current_len += next_syllable_len;
        } else {
            break;
        }
    }

    name
}

#[cfg(test)]
mod tests {
    use crate::make_name;

    #[test]
    fn it_works() {
        println!(
            "A couple of names: '{}', '{}', '{}', '{}'",
            &make_name(0),
            &make_name(2),
            &make_name(5),
            &make_name(100),
        );
        // panic!()
    }
}
