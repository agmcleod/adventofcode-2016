extern crate read_input;

#[derive(Copy, Clone, Debug, PartialEq)]
enum ComponentType {
    Chip, Generator
}

#[derive(Copy, Clone, Debug)]
struct Component<'a> {
    name: &'a str,
    component_type: ComponentType,
}

struct Elevator<'a> {
    slot_one: Option<Component<'a>>,
    slot_two: Option<Component<'a>>,
}

type Floor<'a> = [Option<Component<'a>>; 10];

fn floor_is_safe(floor: &Floor, elevator: Option<Elevator>) -> bool {
    let mut components_to_compare: Vec<Component> = Vec::with_capacity(floor.len());

    for slot in floor {
        if let Some(component) = *slot {
            components_to_compare.push(component);
        }
    }

    if let Some(e) = elevator {
        if let Some(slot) = e.slot_one {
            components_to_compare.push(slot);
        }
        if let Some(slot) = e.slot_two {
            components_to_compare.push(slot);
        }
    }

    components_to_compare.sort_by(|a, b| a.name.cmp(b.name));
    let generator_count = components_to_compare.iter().fold(0, |sum, component|
        if component.component_type == ComponentType::Generator {
            sum + 1
        } else {
            sum
        }
    );

    println!("{:?}", components_to_compare);

    let mut it = components_to_compare.iter();
    let mut component_one: Option<Component> = None;

    let mut is_safe = true;
    let mut component_one_set = false;
    loop {
        if component_one_set {
            component_one_set = false;
        } else {
            component_one = match it.next() {
                Some(c) => Some(*c),
                None => break,
            };
        }

        let component_two = match it.next() {
            Some(c) => Some(*c),
            None => {
                if let Some(c1) = component_one {
                    if c1.component_type == ComponentType::Chip && generator_count > 0 {
                        is_safe = false;
                    }
                }
                break
            },
        };

        if let (Some(c1), Some(c2)) = (component_one, component_two) {
            if c1.name != c2.name {
                if c1.component_type == ComponentType::Chip && generator_count > 0 {
                    is_safe = false;
                    break
                } else {
                    component_one = Some(c2);
                    component_one_set = true;
                }
            }
        }
    }

    is_safe
}

fn main() {
    let text = match read_input::read_text("input.txt") {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e),
    };

    let text = text.replace(",", "");
    let text = text.replace(".", "");

    let mut floors: [Floor; 4] = [[None; 10]; 4];

    let mut column_index = 0;
    for (floor_index, line) in text.lines().enumerate() {
        let mut words = line.split(" ").skip_while(|&w| w != "a" && w != "nothing");
        if let Some(word) = words.next() {
            if word == "nothing" {
                continue
            }
        } else {
            continue
        }

        while let Some(word) = words.next() {
            if word == "generator" || word == "a" || word == "and" || word == "microchip" {
                continue
            }
            let name;
            let component_type = if word.contains("-") {
                name = word.split("-").next().unwrap();
                ComponentType::Chip
            } else {
                name = word;
                ComponentType::Generator
            };

            floors[floor_index][column_index] = Some(Component{ name: name, component_type: component_type });

            column_index += 1;
        }
    }

    for floor in floors.iter() {
        println!("{:?}", floor);
    }
}

#[test]
fn test_floor_is_safe() {
    let mut floor: Floor = [None; 10];
    assert_eq!(floor_is_safe(&floor, None), true);

    floor[0] = Some(Component{ name: "one", component_type: ComponentType::Chip });
    floor[1] = Some(Component{ name: "one", component_type: ComponentType::Generator });
    assert_eq!(floor_is_safe(&floor, None), true);

    floor[0] = Some(Component{ name: "one", component_type: ComponentType::Chip });
    floor[1] = Some(Component{ name: "two", component_type: ComponentType::Generator });
    assert_eq!(floor_is_safe(&floor, None), false);

    floor[0] = Some(Component{ name: "one", component_type: ComponentType::Chip });
    floor[1] = Some(Component{ name: "two", component_type: ComponentType::Chip });
    floor[2] = Some(Component{ name: "two", component_type: ComponentType::Generator });
    assert_eq!(floor_is_safe(&floor, None), false);

    floor[0] = Some(Component{ name: "one", component_type: ComponentType::Generator });
    floor[1] = Some(Component{ name: "two", component_type: ComponentType::Chip });
    floor[2] = Some(Component{ name: "two", component_type: ComponentType::Generator });
    assert_eq!(floor_is_safe(&floor, None), true);

    floor[0] = Some(Component{ name: "one", component_type: ComponentType::Generator });
    floor[1] = Some(Component{ name: "one", component_type: ComponentType::Chip });
    floor[2] = Some(Component{ name: "two", component_type: ComponentType::Chip });
    floor[3] = Some(Component{ name: "two", component_type: ComponentType::Generator });
    floor[4] = Some(Component{ name: "three", component_type: ComponentType::Chip });
    assert_eq!(floor_is_safe(&floor, None), false);

    floor[0] = Some(Component{ name: "one", component_type: ComponentType::Generator });
    floor[1] = Some(Component{ name: "one", component_type: ComponentType::Chip });
    floor[2] = Some(Component{ name: "two", component_type: ComponentType::Chip });
    floor[3] = Some(Component{ name: "two", component_type: ComponentType::Generator });
    floor[4] = Some(Component{ name: "three", component_type: ComponentType::Generator });
    assert_eq!(floor_is_safe(&floor, None), true);

    floor[0] = Some(Component{ name: "one", component_type: ComponentType::Generator });
    floor[1] = Some(Component{ name: "one", component_type: ComponentType::Chip });
    floor[2] = Some(Component{ name: "two", component_type: ComponentType::Chip });
    floor[3] = Some(Component{ name: "two", component_type: ComponentType::Generator });
    floor[4] = Some(Component{ name: "three", component_type: ComponentType::Generator });
    let elevator = Some(Elevator{
        slot_one: Some(Component{ name: "four", component_type: ComponentType::Generator }),
        slot_two: None,
    });
    assert_eq!(floor_is_safe(&floor, elevator), true);

    floor[0] = Some(Component{ name: "one", component_type: ComponentType::Generator });
    floor[1] = Some(Component{ name: "one", component_type: ComponentType::Chip });
    floor[2] = Some(Component{ name: "two", component_type: ComponentType::Chip });
    floor[3] = Some(Component{ name: "two", component_type: ComponentType::Generator });
    floor[4] = Some(Component{ name: "three", component_type: ComponentType::Generator });
    let elevator = Some(Elevator{
        slot_one: Some(Component{ name: "four", component_type: ComponentType::Chip }),
        slot_two: None,
    });
    assert_eq!(floor_is_safe(&floor, elevator), false);

    floor[0] = Some(Component{ name: "one", component_type: ComponentType::Generator });
    floor[1] = Some(Component{ name: "one", component_type: ComponentType::Chip });
    floor[2] = Some(Component{ name: "two", component_type: ComponentType::Chip });
    floor[3] = Some(Component{ name: "two", component_type: ComponentType::Generator });
    floor[4] = Some(Component{ name: "three", component_type: ComponentType::Generator });
    let elevator = Some(Elevator{
        slot_one: Some(Component{ name: "four", component_type: ComponentType::Chip }),
        slot_two: Some(Component{ name: "four", component_type: ComponentType::Generator }),
    });
    assert_eq!(floor_is_safe(&floor, elevator), true);
}