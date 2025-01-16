use std::collections::HashMap;

#[derive(Debug)]
pub struct Map {
    nodes: HashMap<String, Node>,
    edges: HashMap<String, Vec<String>>
}

#[derive(Debug)]
pub struct Node {
    id: String,
    passable: bool,
    terrain_cost: f32
}


impl Map {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new()
        }
    }
    pub fn add_node(&mut self, id: String, passable: bool, terrain_cost: f32) {
        if self.nodes.contains_key(&id) {
            eprintln!("ERROR: node already exist");
            return;
        } else {
            self.nodes.insert(id.clone(), Node {
                id,
                passable,
                terrain_cost
            });
        }
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        if !self.nodes.contains_key(from) {
            eprintln!("ERROR: node '{}' does not exist", from);
            return;
        } else if !self.nodes.contains_key(to) {
            eprintln!("ERROR: node '{}' does not exist", to);
            return;
        }
        self.edges.entry(from.to_string())
            .or_insert_with(Vec::new)
            .push(to.to_string());

        self.edges.entry(to.to_string())
            .or_insert_with(Vec::new)
            .push(from.to_string());
    }

    pub fn get_neighbors(&self,  node_id: &str) -> Option<&Vec<String>> {
        self.edges.get(node_id)
    }

    pub fn is_passable(&self, node_id: &str) -> Option<bool> {
        self.nodes.get(node_id).map(|node| node.passable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node() {
        let mut map = Map::new();
        map.add_node("A".to_string(), true, 1.0);

        assert!(map.nodes.contains_key("A"));
        assert_eq!(map.nodes.get("A").unwrap().passable, true);
        assert_eq!(map.nodes.get("A").unwrap().terrain_cost, 1.0);
    }

    #[test]
    fn test_add_duplicate_node() {
        let mut map = Map::new();
        map.add_node("A".to_string(), true, 1.0);
        map.add_node("A".to_string(), false, 2.0);

        // Узел "A" не должен перезаписаться
        assert_eq!(map.nodes.get("A").unwrap().passable, true);
        assert_eq!(map.nodes.get("A").unwrap().terrain_cost, 1.0);
    }

    #[test]
    fn test_add_edge() {
        let mut map = Map::new();
        map.add_node("A".to_string(), true, 1.0);
        map.add_node("B".to_string(), true, 1.0);

        map.add_edge("A", "B");

        assert!(map.edges.contains_key("A"));
        assert!(map.edges.get("A").unwrap().contains(&"B".to_string()));

        // Проверка для неориентированного графа
        assert!(map.edges.contains_key("B"));
        assert!(map.edges.get("B").unwrap().contains(&"A".to_string()));
    }

    #[test]
    fn test_add_edge_missing_node() {
        let mut map = Map::new();
        map.add_node("A".to_string(), true, 1.0);

        // Пытаемся добавить ребро к несуществующему узлу
        map.add_edge("A", "B");
        assert!(map.edges.get("A").is_none());
    }

    #[test]
    fn test_get_neighbors() {
        let mut map = Map::new();
        map.add_node("A".to_string(), true, 1.0);
        map.add_node("B".to_string(), true, 1.0);
        map.add_node("C".to_string(), true, 1.0);

        map.add_edge("A", "B");
        map.add_edge("A", "C");

        // Проверяем соседей для "A"
        let neighbors_a = map.get_neighbors("A").unwrap();
        assert!(neighbors_a.contains(&"B".to_string()));
        assert!(neighbors_a.contains(&"C".to_string()));

        // Проверяем соседей для "B"
        let neighbors_b = map.get_neighbors("B").unwrap();
        assert!(neighbors_b.contains(&"A".to_string())); // Если граф неориентированный

        // Проверяем узел без соседей
        map.add_node("D".to_string(), true, 1.0);
        assert!(map.get_neighbors("D").unwrap_or(&vec![]).is_empty());
    }


    #[test]
    fn test_is_passable() {
        let mut map = Map::new();
        map.add_node("A".to_string(), true, 1.0);
        map.add_node("B".to_string(), false, 1.0);

        assert_eq!(map.is_passable("A"), Some(true));
        assert_eq!(map.is_passable("B"), Some(false));
        assert_eq!(map.is_passable("C"), None); // Узел "C" не существует
    }

    #[test]
    fn test_complex_map() {
        let mut map = Map::new();
        map.add_node("A".to_string(), true, 1.0);
        map.add_node("B".to_string(), true, 1.5);
        map.add_node("C".to_string(), false, 2.0);
        map.add_node("D".to_string(), true, 0.5);

        map.add_edge("A", "B");
        map.add_edge("B", "C");
        map.add_edge("C", "D");
        map.add_edge("A", "D");

        // Проверка соседей
        assert!(map.get_neighbors("A").unwrap().contains(&"B".to_string()));
        assert!(map.get_neighbors("A").unwrap().contains(&"D".to_string()));

        // Проверка проходимости
        assert_eq!(map.is_passable("A"), Some(true));
        assert_eq!(map.is_passable("C"), Some(false));
    }
}

