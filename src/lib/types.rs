pub struct Grid {
    pub grid: Vec<Vec<i8>>,
}

pub struct Ant {
    pub position: (u32, u32),
    pub direction: i8,
}

impl Ant {
    pub fn new(position: (u32, u32), direction: i8) -> Self {
        Ant {
            position,
            direction,
        }
    }

    pub fn move_ant(&mut self, box_state: i8) {
        // If the state of the box is 1 the ant will move 90° to the right.
        // If the state of the box is 0 the ant will move 90° to the left.
        let rotate = if box_state == 1 { -1 } else { 1 };
        // Chage ant's direction
        self.direction = (self.direction + rotate).rem_euclid(4);

        // update the ant's position based on its direction
        // 0 = North
        // 1 = EAST
        // 2 = SOUTH
        // 3 = WEST
        
        match self.direction {
            0 => self.position.1 += 1,
            1 => self.position.0 += 1,
            2 => self.position.1 -= 1,
            3 => self.position.0 -= 1,
            _ => (),
        };
    }
}

impl Grid {
    pub fn new(rows: u32, columns: u32) -> Self {
        let mut grid_vector = Vec::new();

        for row in 0..rows {
            grid_vector.push(Vec::new());
            for _column in 0..columns {
                grid_vector[row as usize].push(0i8);
            }
        }

        Grid { grid: grid_vector }
    }
}
