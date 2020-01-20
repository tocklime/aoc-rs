/*The first floor contains a promethium generator and a promethium-compatible microchip.
The second floor contains a cobalt generator, a curium generator, a ruthenium generator, and a plutonium generator.
The third floor contains a cobalt-compatible microchip, a curium-compatible microchip, a ruthenium-compatible microchip, and a plutonium-compatible microchip.
The fourth floor contains nothing relevant.
*/

enum Item<'a> {
    Microchip(&'a str),
    Generator(&'a str),
}

impl<'a> Item<'a> {
    fn are_compatible(&self, other: &Self) -> bool {
        match (self, other) {
            (Item::Microchip(a), Item::Generator(b)) => a == b,
            (Item::Generator(a), Item::Microchip(b)) => a == b,
            _ => true
        }
    }
}

struct World<'a> {
    elevator: usize,
    floors: Vec<Vec<Item<'a>>>,
}

impl<'a> World<'a> {
}
