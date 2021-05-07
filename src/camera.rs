use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new(player_position: Point) -> Self {
        let mut cam = Camera {
            left_x: player_position.x - DISPLAY_WIDTH / 2,
            right_x: player_position.x + DISPLAY_WIDTH / 2,
            top_y: player_position.y - DISPLAY_HEIGTH / 2,
            bottom_y: player_position.y + DISPLAY_HEIGTH / 2,
        };
        cam.bound_to_screen();
        cam
    }
    pub fn on_player_move(&mut self, player_position: Point) {
        self.left_x = player_position.x - DISPLAY_WIDTH / 2;
        self.right_x = player_position.x + DISPLAY_WIDTH / 2;
        self.top_y = player_position.y - DISPLAY_HEIGTH / 2;
        self.bottom_y = player_position.y + DISPLAY_HEIGTH / 2;
        self.bound_to_screen();
    }
    fn bound_to_screen(&mut self) {
        // check camera boundaries...
        if self.left_x < 0 {
            self.right_x -= self.left_x;
            self.left_x = 0;
        }
        if self.right_x >= SCREEN_WIDTH {
            self.left_x -= self.right_x - SCREEN_WIDTH + 1;
            self.right_x = SCREEN_WIDTH - 1;
        }
        if self.top_y < 0 {
            self.bottom_y -= self.top_y;
            self.top_y = 0;
        }
        if self.bottom_y >= SCREEN_HEIGTH {
            self.top_y -= self.bottom_y - SCREEN_HEIGTH + 1;
            self.bottom_y = SCREEN_HEIGTH - 1;
        }
    }
}
