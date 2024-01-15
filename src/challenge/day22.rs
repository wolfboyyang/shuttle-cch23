use std::collections::HashMap;

use axum::{response::IntoResponse, routing::post, Router};
use glam::{IVec3, UVec2};
use pathfinding::directed::bfs::bfs;

pub fn task() -> Router {
    Router::new()
        .route("/integers", post(get_present))
        .route("/rocket", post(get_path))
}

async fn get_present(text: String) -> impl IntoResponse {
    let dict = text
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|num| num.trim().parse::<u64>().unwrap())
        .fold(HashMap::new(), |mut dict, num| {
            if dict.contains_key(&num) {
                dict.remove_entry(&num);
            } else {
                dict.insert(num, ());
            }
            dict
        });

    let ord_num = *dict.keys().next().unwrap();

    "🎁".repeat(ord_num as usize)
}

async fn get_path(text: String) -> impl IntoResponse {
    let mut input = text
        .lines()
        .map(|line| line.trim())
        //.inspect(|line| println!("Processing line: {}", line))
        .filter(|line| !line.is_empty());

    let num_of_stars = input.next().unwrap().parse::<usize>().unwrap();

    let map = input
        .clone()
        .take(num_of_stars as usize)
        //.inspect(|line| println!("convert map: {}", line))
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|coords| IVec3::new(coords[0], coords[1], coords[2]))
        .collect::<Vec<_>>();

    let mut input = input.skip(num_of_stars).clone();

    let num_of_portals = input.next().unwrap().parse::<usize>().unwrap();

    let portals = input
        .take(num_of_portals)
        //.inspect(|line| println!("convert connection: {}", line))
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|connection| UVec2::new(connection[0], connection[1]))
        .collect::<Vec<_>>();

    let path = bfs(
        &0,
        |p: &usize| {
            portals
                .iter()
                .filter(|c| c.x as usize == *p)
                .map(|c| c.y as usize)
                .collect::<Vec<_>>()
        },
        |p| *p == num_of_stars - 1,
    )
    .unwrap();

    let distance = path.windows(2).fold(0.0, |acc, p| {
        acc + ((map[p[0]] - map[p[1]]).length_squared() as f32).sqrt()
    });

    format!("{} {:.3}", path.len() - 1, distance)
}
