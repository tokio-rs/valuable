use std::collections::HashMap;

use valuable::*;
use valuable_serde::Serializable;

#[derive(Valuable, serde::Serialize)]
struct HelloWorld {
    hello: &'static str,
    world: World,
    list: Vec<World>,
    map: HashMap<&'static str, u32>,
    bool_: bool,
    unit: (),
    enums: Vec<Enum>,
}

#[derive(Valuable, serde::Serialize)]
struct World {
    answer: usize,
}

#[derive(Valuable, serde::Serialize)]
enum Enum {
    Struct { x: &'static str },
    Newtype(u8),
    Tuple(u8, u16),
    // TODO
    // Unit,
}

#[test]
fn test() {
    let hello_world = HelloWorld {
        hello: "wut",
        world: World { answer: 42 },
        list: vec![World { answer: 1 }, World { answer: 2 }],
        map: {
            let mut m = HashMap::new();
            m.insert("k", 0);
            m
        },
        bool_: false,
        unit: (),
        enums: vec![
            Enum::Struct { x: "a" },
            Enum::Newtype(0),
            Enum::Tuple(0, 1),
            // Enum::Unit,
        ],
    };

    let value = Serializable::new(&hello_world);
    assert_eq!(
        serde_json::to_string(&value).unwrap(),
        serde_json::to_string(&hello_world).unwrap(),
    );

    // println!("{}", serde_json::to_string_pretty(&value).unwrap());
    assert_eq!(
        serde_json::to_string_pretty(&value).unwrap(),
        r#"{
  "hello": "wut",
  "world": {
    "answer": 42
  },
  "list": [
    {
      "answer": 1
    },
    {
      "answer": 2
    }
  ],
  "map": {
    "k": 0
  },
  "bool_": false,
  "unit": null,
  "enums": [
    {
      "Struct": {
        "x": "a"
      }
    },
    {
      "Newtype": 0
    },
    {
      "Tuple": [
        0,
        1
      ]
    }
  ]
}"#
    );
}
