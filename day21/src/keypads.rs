use pathfinding::prelude::dijkstra;
use std::collections::HashMap; // Import dijkstra from the pathfinding crate

#[derive(Debug)]
pub struct Keypad {
    pub numerical: HashMap<char, Vec<(char, char)>>,
    pub directional: HashMap<char, Vec<(char, char)>>,
}

impl Keypad {
    pub fn new() -> Self {
        let mut numerical = HashMap::new();

        // First row
        numerical.insert('7', vec![('8', '>'), ('4', 'v')]);
        numerical.insert('8', vec![('7', '<'), ('9', '>'), ('5', 'v')]);
        numerical.insert('9', vec![('8', '<'), ('6', 'v')]);

        // Second row
        numerical.insert('4', vec![('7', '^'), ('5', '>'), ('1', 'v')]);
        numerical.insert('5', vec![('8', '^'), ('4', '<'), ('6', '>'), ('2', 'v')]);
        numerical.insert('6', vec![('9', '^'), ('5', '<'), ('3', 'v')]);

        // Third row
        numerical.insert('1', vec![('4', '^'), ('2', '>')]);
        numerical.insert('2', vec![('5', '^'), ('1', '<'), ('3', '>'), ('0', 'v')]);
        numerical.insert('3', vec![('6', '^'), ('2', '<'), ('A', 'v')]);

        // Fourth row
        numerical.insert('0', vec![('2', '^'), ('A', '>')]);
        numerical.insert('A', vec![('3', '^'), ('0', '<')]);

        let mut directional = HashMap::new();
        directional.insert('A', vec![('>', 'v'), ('^', '<')]);
        directional.insert('^', vec![('A', '>'), ('v', 'v')]);
        directional.insert('<', vec![('v', '>')]);
        directional.insert('v', vec![('>', '>'), ('<', '<')]);
        directional.insert('>', vec![('A', '^'), ('v', '<')]);

        Keypad {
            numerical,
            directional,
        }
    }

    pub fn path_directional_to_numerical(&self, code: &String) -> Vec<char> {
        let mut result: Vec<char> = Vec::new();
        let mut start = 'A';
        result.push(start);

        code.chars().for_each(|c| {
            let path: Option<(Vec<char>, usize)> = dijkstra(
                &start,
                |p| {
                    self.numerical
                        .get(&p)
                        .unwrap()
                        .iter()
                        .map(|(c, _)| (*c, 1))
                        .collect::<Vec<(char, usize)>>()
                },
                |p| *p == c,
            );
            println!("Path: {:?}", path);
            result.extend(path.unwrap_or_else(|| (Vec::new(), 0)).0[1..].iter());
            start = c;
        });

        println!("Result: {:?}", result);
        result
    }
}
