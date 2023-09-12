use crate::config::{self, Hardware};

pub fn hardware1() -> Hardware {
    Hardware {
        controls: vec![
            config::Control {
                name: "control1".into(),
            },
            config::Control {
                name: "control2".into(),
            },
            config::Control {
                name: "control3".into(),
            },
            config::Control {
                name: "control4".into(),
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
    }
}

pub fn config1() -> config::Config {
    config::Config {
        behaviors: vec![
            config::Behavior::TempMath(config::TempMath {
                name: "max".into(),
                kind: config::TempMathType::Max,
                input: vec!["temp1".into(), "temp2".into()],
            }),
            config::Behavior::Graph(config::Graph {
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
                output: vec!["control1".into()],
            }),
            config::Behavior::Flat(config::Flat {
                name: "flat1".into(),
                value: 50,
                output: vec!["control2".into()],
            }),
            config::Behavior::Linear(config::Linear {
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
                output: vec!["control3".into()],
            }),
            config::Behavior::Target(config::Target {
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
                output: vec!["control4".into()],
            }),
        ],
    }
}
