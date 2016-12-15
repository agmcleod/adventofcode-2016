extern crate read_input;

#[derive(Copy, Clone, Debug)]
enum ComponentType {
    Chip, Generator
}

#[derive(Copy, Clone, Debug)]
struct Component<'a> {
    name: &'a str,
    component_type: ComponentType,
}

fn main() {
    let text = match read_input::read_text("input.txt") {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e),
    };

    let text = text.replace(",", "");
    let text = text.replace(".", "");

    let mut floors: [[Option<Component>; 10]; 4] = [[None; 10]; 4];

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
