use state_machines::state_machine;

// Define your state machine
state_machine! {
    name: TrafficLight,
    dynamic: true,
    initial: Red,
    states: [Red, Yellow, Green],
    events {
        next {
            transition: { from: Red, to: Green }
            transition: { from: Green, to: Yellow }
            transition: { from: Yellow, to: Red }
        }
    }
}