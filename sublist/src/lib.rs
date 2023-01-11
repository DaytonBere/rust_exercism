use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {

    let mut sub: bool = true;
    let first_count: HashMap<i32, i32> = HashMap::new();

    for a in _first_list.into_iter() {
        first_count.insert(a, first_count.entry(a).or_insert(0)+1);
    }

    for (key, value) in &first_count {
        println!("{}: {}", key, value);
    }
    
    for a in _first_list.into_iter() {
        let mut found: bool = false;
        
        for b in _second_list.into_iter() {
            if a == b {
                found = true;
                break;
            }
        }

        if !found {
            sub = false;
            break;
        }
    }

    let mut sup: bool = true;
    
    for b in _second_list.into_iter() {
        let mut found: bool = false;
        
        for a in _first_list.into_iter() {
            if a == b {
                found = true;
                break;
            }
        }

        if !found {
            sup = false;
            break;
        }
    }


    if sub && sup {
        return Comparison::Equal;
    } else if sub {
        return Comparison::Sublist;
    } else if sup {
        return Comparison::Superlist;
    } else {
        return Comparison::Unequal;
    }
}
