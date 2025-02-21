use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use serde_json::Value;

pub fn count_provinces() -> String {
    // 读取并解析JSON文件
    let mut file = File::open("district.json").expect("Failed to open district.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let json: Value = serde_json::from_str(&contents).unwrap();

    let mut results = Vec::with_capacity(5);

    // 处理每个批次
    for i in 1..=5 {
        let batch = &json[i.to_string()];
        if batch.is_object() {
            let count = count_connected_components(batch);
            results.push(count);
        } else {
            results.push(0); // 处理无效的批次数据
        }
    }

    // 将结果转换为指定格式的字符串
    results.iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn count_connected_components(batch: &Value) -> i32 {
    let batch_obj = match batch.as_object() {
        Some(obj) => obj,
        None => return 0, // 非对象类型直接返回0
    };

    // 当前批次的所有城市
    let cities_in_batch: HashSet<&str> = batch_obj.keys().map(|k| k.as_str()).collect();
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

    // 构建图，仅处理当前批次内的连接
    for (city, connections) in batch_obj {
        let city_str = city.as_str();
        if let Some(conn_array) = connections.as_array() {
            for conn in conn_array.iter().filter_map(|v| v.as_str()) {
                // 确保连接的城市属于当前批次且排除自环
                if cities_in_batch.contains(conn) && city_str != conn {
                    // 添加双向边
                    graph.entry(city_str.to_string())
                        .or_insert_with(HashSet::new)
                        .insert(conn.to_string());
                    graph.entry(conn.to_string())
                        .or_insert_with(HashSet::new)
                        .insert(city_str.to_string());
                }
            }
        }
    }

    let mut visited = HashSet::new();
    let mut provinces = 0;

    // 遍历批次中的所有城市，确保孤立城市也被计数
    for &city in &cities_in_batch {
        let city_str = city.to_string();
        if !visited.contains(&city_str) {
            dfs(&graph, &city_str, &mut visited);
            provinces += 1;
        }
    }

    provinces
}

// 使用迭代DFS避免栈溢出
fn dfs(graph: &HashMap<String, HashSet<String>>, start: &str, visited: &mut HashSet<String>) {
    let mut stack = vec![start.to_string()];
    while let Some(city) = stack.pop() {
        if !visited.insert(city.clone()) {
            continue;
        }
        if let Some(neighbors) = graph.get(&city) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    stack.push(neighbor.to_string());
                }
            }
        }
    }
}