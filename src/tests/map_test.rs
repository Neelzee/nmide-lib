use rstest::rstest;

use crate::model::map::{Map, Value};

#[rstest]
#[case(vec![], vec![], vec![])]
#[case(vec![], vec!["something"], vec![None])]
#[case(
    vec![("something", Value::String("".to_string()))],
    vec!["something"],
    vec![Some(Value::String("".to_string()))]
)]
fn test_map_adding_getting(
    #[case] adding: Vec<(&str, Value)>,
    #[case] getting: Vec<&str>,
    #[case] expecting: Vec<Option<Value>>,
) {
    let mut map = Map::new();

    for (k, v) in adding {
        assert!(map.insert(k.to_string(), v).is_none());
    }

    let mut gotten = Vec::new();

    for k in getting {
        gotten.push(map.get(&(k.to_string())));
    }

    for got in gotten {
        assert!(expecting.contains(&got))
    }
}

#[rstest]
#[case(vec![], vec![], vec![])]
#[case(vec![("something", Value::Bool(true))], vec![], vec![Value::Bool(true)])]
#[case(vec![("something", Value::Bool(true))], vec!["something"], vec![])]
#[case(vec![("something", Value::Bool(true))], vec![""], vec![Value::Bool(true)])]
fn test_map_removing(
    #[case] adding: Vec<(&str, Value)>,
    #[case] removing: Vec<&str>,
    #[case] expecting: Vec<Value>,
) {
    let mut map = Map::new();

    for (k, v) in adding {
        assert!(map.insert(k.to_string(), v).is_none());
    }

    for k in removing {
        let _ = map.remove(&(k.to_string())).is_some();
    }

    for (_, v) in map {
        assert!(expecting.contains(&v));
    }
}
