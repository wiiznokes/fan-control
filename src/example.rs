use crate::config;

pub fn config1() -> config::Config {
    config::Config {
        unit: config::Unit::C,
        fans: vec![
            config::Fan {
                name: "fan1".into(),
            },
            config::Fan {
                name: "fan2".into(),
            },
            config::Fan {
                name: "fan3".into(),
            },
            config::Fan {
                name: "fan4".into(),
            },
        ],
        temps: vec![
            config::Temp {
                name: "temp1".into(),
            },
            config::Temp {
                name: "temp2".into(),
            },
            config::Temp {
                name: "temp3".into(),
            },
        ],
        controls: vec![
            config::Control::TempMath(config::TempMath {
                name: "max".into(),
                kind: config::TempMathType::Max,
                input: vec!["temp1".into(), "temp2".into()],
            }),
            config::Control::Graph(config::Graph {
                name: "graph1".into(),
                coord: vec![
                    config::Coord {
                        temp: 10,
                        percent: 10,
                    },
                    config::Coord {
                        temp: 50,
                        percent: 30,
                    },
                    config::Coord {
                        temp: 90,
                        percent: 100,
                    },
                ],
                input: "max".into(),
                output: vec!["fan1".into()],
            }),
            config::Control::Flat(config::Flat {
                name: "flat1".into(),
                value: 50,
                output: vec!["fan2".into()],
            }),
            config::Control::Linear(config::Linear {
                name: "graph1".into(),
                min: config::Coord {
                    temp: 10,
                    percent: 10,
                },
                max: config::Coord {
                    temp: 70,
                    percent: 100,
                },
                input: "temp3".into(),
                output: vec!["fan3".into()],
            }),
            config::Control::Target(config::Target {
                name: "graph1".into(),
                ideal: config::Coord {
                    temp: 40,
                    percent: 10,
                },
                load: config::Coord {
                    temp: 70,
                    percent: 100,
                },
                input: "temp3".into(),
                output: vec!["fan4".into()],
            }),
        ],
    }
}
