use evdev::{EventType, PropType};

fn main() {
    let devices = evdev::enumerate();
    for (_path, device) in devices {
        let name = device.name();
        let unique_name = device.unique_name();
        let properties = device.properties();
        let events = device.supported_events();

        let mut heuristic = 5;

        if let Some(name) = name {
            heuristic -= name.to_lowercase().contains("keyboard") as i8;
        }

        if let Some(name) = unique_name {
            heuristic -= name.to_lowercase().contains("keyboard") as i8;
        }

        // properties a keyboard generally has:
        // keys
        heuristic -= events.contains(EventType::KEY) as i8;
        // leds (capslocks, numpads)
        heuristic -= events.contains(EventType::LED) as i8;
        // repeat (holding a key to spam it?)
        heuristic -= events.contains(EventType::REPEAT) as i8;

        heuristic += events.contains(EventType::ABSOLUTE) as i8;
        heuristic += events.contains(EventType::RELATIVE) as i8;
        heuristic += events.contains(EventType::SWITCH) as i8;
        heuristic += properties.contains(PropType::POINTER) as i8;
        heuristic += properties.contains(PropType::BUTTONPAD) as i8;

        println!("name: {:?}, heuristic: {}", name, heuristic);
    }
}
