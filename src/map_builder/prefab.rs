use crate::prelude::*;

struct FortressStruct<'a> {
    map_str: &'a str,
    x: i32,
    y: i32,
}

const FORTRESS: FortressStruct = FortressStruct {
    map_str: "
------------
---######---
---#--A-#---
---#-M--#---
-###----###-
--M------M--
-###----###-
---#----#---
---#----#---
---######---
------------
",
    x: 12,
    y: 11,
};

pub fn apply(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    let mut placement = None;

    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &vec![mb.map.point2d_to_index(mb.player_start)],
        &mb.map,
        1024.0,
    );

    let mut attempts = 0;
    while placement.is_none() && attempts < 10 {
        let dimensions = Rect::with_size(
            rng.range(0, SCREEN_WIDTH - FORTRESS.x),
            rng.range(0, SCREEN_HEIGHT - FORTRESS.y),
            FORTRESS.x,
            FORTRESS.y,
        );

        #[cfg(debug_assertions)]
        mb.map.dump(Some(dimensions));

        let can_place = dimensions.point_set().iter().any(|pt| {
            let idx = mb.map.point2d_to_index(*pt);
            let distance = dijkstra_map.map[idx];
            distance < 2000.0 && distance > 50.0 //&& *pt != mb.amulet_start
        });
        if can_place {
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            mb.monster_spawns.retain(|pt| !points.contains(pt));
        }
        attempts += 1;
    }

    if let Some(placement) = placement {
        let string_vec: Vec<char> = FORTRESS
            .map_str
            .chars()
            .filter(|a| *a != '\r' && *a != '\n')
            .collect();
        // remove old monsters
        mb.monster_spawns.clear();
        let mut i = 0;
        for ty in placement.y..placement.y + FORTRESS.y {
            for tx in placement.x..placement.x + FORTRESS.x {
                let idx = get_idx(tx, ty);
                let c = string_vec[i];
                match c {
                    'M' => {
                        mb.map.tiles[idx] = TileType::Floor;
                        mb.monster_spawns.push(Point::new(tx, ty));
                    }
                    'A' => {
                        mb.map.tiles[idx] = TileType::Floor;
                        mb.amulet_start = Point::new(tx, ty);
                    }
                    '-' => mb.map.tiles[idx] = TileType::Floor,
                    '#' => mb.map.tiles[idx] = TileType::Wall,
                    _ => {
                        mb.map.tiles[idx] = TileType::Floor;
                        println!("No idea what to do with it[{}], placed floor instead", c);
                    }
                }
                i += 1;
            }
        }
        // spawn other monsters
        mb.monster_spawns = mb.spawn_monsters(&mb.player_start, rng);
    } else {
        println!("No success in placing the Fortress");
    }
}
