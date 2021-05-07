use crate::prelude::*;
const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };
        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.dig_corridors(rng);
        mb.player_start = mb.rooms[0].center();
        mb
    }
    // -----------------------------------------------------------
    // Private methods
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }
    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGTH - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH
                        && p.y > 0 && p.y < SCREEN_HEIGTH
                    {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                println!("ADDED ROOM WITH CENTER {}.{}", room.center().x, room.center().y);
                self.rooms.push(room);
            }
        }
    }
    fn dig_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{min, max};

        println!("    - Vertical tunnel from {} to {}",
            min(y1, y2), max(y1, y2));

        for y in min(y1, y2)..= max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }
    fn dig_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{min, max};

        println!("    - Horizontal tunnel from {} to {}",
            min(x1, x2), max(x1, x2));

        for x in min(x1, x2)..= max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }
    fn dig_corridors(&mut self, rng: &mut RandomNumberGenerator) {

        // We need to clone to allow ordering with sort_by()
        let mut rooms = self.rooms.clone();

        // Sort the vector of Rect using the x coordinate of the
        // rectangle center using the an_i32.cmp(another_i32)
        // where:
        //   - an_i32 = a_rect.center().x and
        //   - another_i32 = another_rect.center().x
        // ----------------------------------------------------------
        // fn cmp(&self, other: &Self) -> Ordering
        // https://doc.rust-lang.org/std/cmp/enum.Ordering.html
        //
        // This method returns an [Ordering] between self and other.
        //
        // By convention, self.cmp(&other) returns the ordering
        // matching the expression self <operator> other if true.
        //
        // Examples
        // use std::cmp::Ordering;
        // assert_eq!(5.cmp(&10), Ordering::Less);
        // assert_eq!(10.cmp(&5), Ordering::Greater);
        // assert_eq!(5.cmp(&5), Ordering::Equal);
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {

            println!("Dig corridor from {}.{} to {}.{}",
                rooms[i-1].center().x, rooms[i-1].center().y,
                room.center().x, room.center().y);

            let prev = rooms[i-1].center();
            let new = room.center();
            if rng.range(0, 2) == 1 {
                self.dig_horizontal_tunnel(prev.x, new.x, prev.y);
                self.dig_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.dig_vertical_tunnel(prev.y, new.y, prev.x);
                self.dig_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
}
