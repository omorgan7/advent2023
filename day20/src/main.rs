use std::collections::HashMap;


type Pulse = u8;
const PULSE_LOW : Pulse = 0;
const PULSE_HIGH : Pulse = 1;

enum Either
{
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Tester(Tester)
}

struct Tester
{
}

struct FlipFlop
{
    on: bool,
    last_received: Pulse,
    outputs: Vec<String>
}

struct Conjunction
{
    inputs: Vec<String>,
    outputs: Vec<String>
}

impl Conjunction
{
    fn receive(&mut self,  modules: &HashMap<String, Either>, pulse: &Pulse) -> Option<Pulse> {
        if self.inputs.iter().all(|x| {
            let module = modules.get(x).unwrap();
            match module {
                Either::FlipFlop(flipflop) => {
                    flipflop.last_received == PULSE_HIGH
                },
                _ => panic!()
            }
        }) {
            Some(PULSE_LOW)
        } else {
            Some(PULSE_HIGH)
        }
    }
}

impl FlipFlop
{
    fn new() -> FlipFlop {
        FlipFlop { on: false, last_received: PULSE_LOW, outputs: vec![] }
    }

    fn receive(&mut self, pulse: &Pulse) -> Option<Pulse>
    {
        self.last_received = *pulse;
        match *pulse {
            PULSE_HIGH => None,
            PULSE_LOW => {
                let returned_pulse = if self.on {
                    PULSE_LOW
                } else {
                    PULSE_HIGH
                };

                self.on = !self.on;
                return Some(returned_pulse);
            },
            _ => panic!()
        }
    }
}

fn main() {

    let input = include_str!("../input.txt");
    println!("Hello, world!");
}
