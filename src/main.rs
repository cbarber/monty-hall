use itertools::Itertools;
use rand::{
    distributions::{Distribution, Uniform},
    prelude::{IteratorRandom, ThreadRng},
};
use std::collections::HashMap;
use std::fmt;

const ITERATIONS: usize = 1_000_000;
const DOOR_COUNT: usize = 3;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Door {
    Prize,
    Goat,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Scenario {
    doors: Vec<Door>,
    first_guess: usize,
    reveal_door: usize,
}

impl Scenario {
    fn new(rng: &mut ThreadRng, random_door: Uniform<usize>) -> Self {
        let mut doors = vec![Door::Goat; DOOR_COUNT];
        doors[random_door.sample(rng)] = Door::Prize;

        let first_guess = random_door.sample(rng);

        let reveal_door: usize = doors
            .iter()
            .enumerate()
            .positions(|(index, door)| index != first_guess && *door == Door::Goat)
            .choose(rng)
            .expect("to choose one random goat door");

        Self {
            doors,
            first_guess,
            reveal_door,
        }
    }

    fn first_guess_win(&self) -> bool {
        self.doors[self.first_guess] == Door::Prize
    }

    fn unrevealed_door(&self) -> usize {
        (0..DOOR_COUNT)
            .into_iter()
            .find(|&index| index != self.first_guess && index != self.reveal_door)
            .expect("to find unrevealed door")
    }
}

#[derive(Default)]
struct Outcome {
    occurences: f32,
    first_guess_win: f32,
    remaining_doors_guess_win: f32,
    swap_guess_win: f32,
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "  occurences: {},\n  first_guess_win: {:.1}%,\n  remaining_doors_guess_win: {:.1}%,\n  swap_guess_win: {:.1}%",
            self.occurences,
            self.first_guess_win / self.occurences * 100.0,
            self.remaining_doors_guess_win  / self.occurences * 100.0,
            self.swap_guess_win / self.occurences * 100.0
        )
    }
}

impl Outcome {
    fn update(&mut self, scenario: &Scenario, remaining_doors_guess: usize, swap_guess: usize) {
        self.occurences += 1.0;

        if scenario.doors[scenario.first_guess] == Door::Prize {
            self.first_guess_win += 1.0;
        }

        if scenario.doors[remaining_doors_guess] == Door::Prize {
            self.remaining_doors_guess_win += 1.0;
        }

        if scenario.doors[swap_guess] == Door::Prize {
            self.swap_guess_win += 1.0;
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let random_door = Uniform::from(0..DOOR_COUNT);

    let mut total_outcome = Outcome::default();
    let mut outcomes: HashMap<Scenario, Outcome> = HashMap::new();

    for _ in 0..ITERATIONS {
        let scenario = Scenario::new(&mut rng, random_door);
        let unrevealed_door = scenario.unrevealed_door();

        let remaining_doors_guess = [scenario.first_guess, unrevealed_door]
            .iter()
            .choose(&mut rng)
            .expect("to choose from remaining door")
            .clone();

        total_outcome.update(&scenario, remaining_doors_guess, unrevealed_door);

        if !outcomes.contains_key(&scenario) {
            let mut outcome = Outcome::default();
            outcome.update(&scenario, remaining_doors_guess, unrevealed_door);
            outcomes.insert(scenario, outcome);
        } else {
            outcomes
                .get_mut(&scenario)
                .expect("to get_mut for doors outcome")
                .update(&scenario, remaining_doors_guess, unrevealed_door);
        }
    }

    println!("********************");
    println!("*** First guess wins");
    println!("********************");
    for (scenario, outcome) in outcomes
        .iter()
        .filter(|(scenario, _)| scenario.first_guess_win())
    {
        println!("{:?}", scenario);
        println!("{}", outcome);
        println!("");
    }
    println!("");

    println!("*********************");
    println!("*** First guess loses");
    println!("*********************");
    for (scenario, outcome) in outcomes
        .iter()
        .filter(|(scenario, _)| !scenario.first_guess_win())
    {
        println!("{:?}", scenario);
        println!("{}", outcome);
        println!("");
    }
    println!("");

    println!("Total:");
    println!("{}", total_outcome);
}
