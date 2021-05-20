use std::collections::HashMap;

use valuable::*;

#[derive(Valuable, serde::Serialize)]
struct HelloWorld {
    hello: &'static str,
    world: World,
    list: Vec<World>,
    map: HashMap<&'static str, u32>,
    bool_: bool,
    unit: (),
}

#[derive(Valuable, serde::Serialize)]
struct World {
    answer: usize,
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
    };

    let value = Value::Structable(&hello_world);
    println!("{}", valuable_json::to_string(&value, false));
    assert_eq!(
        valuable_json::to_string(&value, false),
        r#"{"hello":"wut","world":{"answer":42},"list":[{"answer":1},{"answer":2}],"map":{"k":0},"bool_":false,"unit":null}"#
    );
    assert_eq!(
        valuable_json::to_string(&value, false),
        serde_json::to_string(&hello_world).unwrap(),
    );
    println!("{}", valuable_json::to_string(&value, true));
    assert_eq!(
        valuable_json::to_string(&value, true),
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
  "unit": null
}"#
    );
    assert_eq!(
        valuable_json::to_string(&value, true),
        serde_json::to_string_pretty(&hello_world).unwrap(),
    );
}
