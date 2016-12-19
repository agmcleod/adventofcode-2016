extern crate read_input;
extern crate init_with;
use init_with::InitWith;

const NUM_OF_PAIRS: usize = 5;

#[derive(PartialEq)]
enum ComponentType {
    Chip,
    Gen,
}

struct Component {
    name: String,
    c_type: ComponentType,
}

type Floor = Vec<Component>;

fn adjacents() {

}

fn get_groups(floors: &Vec<Floor>) -> [Vec<usize>; NUM_OF_PAIRS] {
    let mut results = {
        let pair = Vec::with_capacity(2);
        <[Vec<usize>; NUM_OF_PAIRS]>::init_with(|| {
            pair.clone()
        })
    };
    let mut names: Vec<String> = Vec::with_capacity(NUM_OF_PAIRS);

    for floor in floors {
        for c in floor {
            if !names.contains(&c.name) {
                names.push(c.name.clone());
            }
        }
    }

    let mut index = 0;
    for name in names {
        let mut pair: Vec<usize> = Vec::with_capacity(2);
        for (i, floor) in floors.iter().enumerate() {
            if floor.iter().filter(|c| c.name == name && c.c_type == ComponentType::Gen).count() > 0 {
                pair.push(i);
            }
            if floor.iter().filter(|c| c.name == name && c.c_type == ComponentType::Chip).count() > 0 {
                pair.push(i);
            }
        }
        results[index] = pair;
        index += 1;
    }

    results
}

fn state_is_invalid(floors: &Vec<Floor>) -> bool {
    for floor in floors {
        let chips: Vec<&Component> = floor.iter().filter(|c| c.c_type == ComponentType::Chip).collect();
        let gens: Vec<&Component> = floor.iter().filter(|c| c.c_type == ComponentType::Gen).collect();
        for chip in chips {
            if gens.len() > 0 && gens.iter().filter(|c| c.c_type == chip.c_type && c.name == chip.name).count() > 0 {
                return true
            }
        }
    }

    false
}

fn main() {
    let text = match read_input::read_text("input.txt") {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e),
    };

    let mut count = 0;

    let floors: Vec<Floor> = text.lines().map(|line| {
        let stuff: Vec<&str> = line.split("a ").map(|bit| bit.trim()).collect();
        let generators: Vec<&str> = stuff.iter().filter(|&s| s.contains("generator")).cloned().collect();
        let chips: Vec<&str> = stuff.iter().filter(|&s| s.contains("microchip")).cloned().collect();

        let mut floor: Floor = Vec::with_capacity(stuff.len());

        for gen in generators {
            floor.push(Component{ name: String::from(gen.split(" ").next().unwrap()), c_type: ComponentType::Gen });
            count += 1;
        }

        for chip in chips {
            floor.push(Component{ name: String::from(chip.split("-").next().unwrap()), c_type: ComponentType::Gen });
            count += 1;
        }

        floor
    }).collect();


}