use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use serde_json::Value;

pub fn count_provinces() -> String {
    // 读取并解析 JSON 数据
    let data = fs::read_to_string("district.json").expect("Unable to read file");
    let graph_input: BTreeMap<String, HashMap<String, Vec<String>>> = serde_json::from_str(&data).expect("Unable to parse JSON");

    graph_count(graph_input)
}

fn build_tree(mut graph: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    // 创建一个新的 HashMap 来存储最终的图
    let mut merged = HashMap::new();

    // 通过原图中的每个城市和其邻居更新新的图
    for (city, neighbors) in graph.into_iter() {
        // 对每个城市，确保它的邻居列表是唯一的并且没有重复
        let adj_list = merged.entry(city.clone()).or_insert_with(Vec::new);
        let mut reverse_neighbors = Vec::new(); // 用来存储反向连接的邻居

        for neighbor in neighbors {
            if !adj_list.contains(&neighbor) {
                adj_list.push(neighbor.clone());
            }

            reverse_neighbors.push(neighbor); // 记录反向连接的邻居
        }

        // 处理反向连接
        for neighbor in reverse_neighbors {
            let reverse_adj_list = merged.entry(neighbor).or_insert_with(Vec::new);
            if !reverse_adj_list.contains(&city) {
                reverse_adj_list.push(city.clone());
            }
        }
    }

    merged
}



fn print_graph(graph: &HashMap<String, Vec<String>>) {
    // 按字母顺序排序所有节点
    let mut sorted_nodes: Vec<_> = graph.keys().collect();
    sorted_nodes.sort_unstable();

    // 遍历每个节点并打印邻接关系
    for node in sorted_nodes {
        let mut neighbors: Vec<_> = graph[node].clone();
        neighbors.sort_unstable();  // 邻接节点也排序
        neighbors.dedup();          // 去重（如果可能有重复）
        
        println!(
            "{} -> {}",
            node,
            if neighbors.is_empty() {
                String::from("(none)")
            } else {
                neighbors.join(", ")
            }
        );
    }
}

// 使用栈实现的 DFS，构建城市间的连接关系
fn dfs(city: &str, graph: &HashMap<String, Vec<String>>, visited: &mut HashSet<String>) {
    let mut stack = vec![city.to_string()];
    visited.insert(city.to_string());

    while let Some(current_city) = stack.pop() {
        if let Some(neighbors) = graph.get(&current_city) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    stack.push(neighbor.clone());
                }
            }
        }
    }
}

// 构建图并计算省份数量
fn graph_count(graph_input: BTreeMap<String, HashMap<String, Vec<String>>>) -> String {
    let mut result = Vec::new();

    // 遍历每个批次的城市群
    for (_, ori_graph) in graph_input {
        let graph = build_tree(ori_graph);
        let mut visited = HashSet::new();
        let mut province_count = 0;

        // 遍历每个城市，检查城市群之间的连接
        for city in graph.keys() {
            if !visited.contains(city) {
                // 通过 DFS 计算所有连通的城市（即一个省）
                dfs(city, &graph, &mut visited);
                province_count += 1;
            }
        }

        result.push(province_count);
    }

    // 将结果转换为以逗号分隔的字符串
    result.iter().map(|&count| count.to_string()).collect::<Vec<String>>().join(",")
}
