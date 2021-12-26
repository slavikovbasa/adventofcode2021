use std::cmp;

#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/23/input";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AType {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl AType {
    fn from_char(c: char) -> Self {
        match c {
            'A' => AType::Amber,
            'B' => AType::Bronze,
            'C' => AType::Copper,
            'D' => AType::Desert,
            _ => panic!("bad type of an amphipod"),
        }
    }

    fn room_no(&self) -> usize {
        match self {
            AType::Amber => 0,
            AType::Bronze => 1,
            AType::Copper => 2,
            AType::Desert => 3,
        }
    }

    fn room_entrance(&self) -> usize {
        (self.room_no() + 1) * 2
    }

    fn step_energy(&self) -> u64 {
        match self {
            AType::Amber => 1,
            AType::Bronze => 10,
            AType::Copper => 100,
            AType::Desert => 1000,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Amphipod {
    typ: AType,
    is_settled: bool,
}

fn is_entrance(idx: usize) -> bool {
    idx == 2 || idx == 4 || idx == 6 || idx == 8
}

#[derive(Debug)]
struct Hallway {
    cells: [Option<Amphipod>; 11],
}

impl Hallway {
    fn new() -> Hallway {
        Hallway { cells: [None; 11] }
    }

    fn put(&self, amphipod: Amphipod, idx: usize) -> Self {
        let mut cells = self.cells.clone();
        if cells[idx].is_some() {
            panic!("can't put amphipod in an occupied hw cell")
        }
        cells[idx] = Some(amphipod);
        Hallway { cells }
    }

    fn drop(&self, idx: usize) -> (Self, Amphipod) {
        let mut cells = self.cells.clone();
        let cell = cells[idx];
        match cell {
            None => panic!("can't drop from empty hw cell"),
            Some(a) => {
                cells[idx] = None;
                (Hallway { cells }, a)
            }
        }
    }

    fn has_path_from_to(&self, from: usize, to: usize) -> bool {
        if from > self.cells.len() || to > self.cells.len() {
            return false;
        }
        let range = if from < to { from + 1..to } else { to..from };

        for i in range {
            if self.cells[i].is_some() {
                return false;
            }
        }
        true
    }

    fn paths_from(&self, from: usize) -> Vec<usize> {
        let mut indexes = Vec::new();
        if from > self.cells.len() {
            return indexes;
        }
        for i in (from + 1)..self.cells.len() {
            if is_entrance(i) {
                continue;
            }
            if self.cells[i].is_some() {
                break;
            }
            indexes.push(i);
        }

        for i in (0..from).rev() {
            if is_entrance(i) {
                continue;
            }
            if self.cells[i].is_some() {
                break;
            }
            indexes.push(i);
        }

        indexes
    }

    fn iter_occupied<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.cells
            .iter()
            .enumerate()
            .filter(|(_, a)| a.is_some())
            .map(|(idx, _)| idx)
    }
}

#[derive(Debug)]
struct Siderooms {
    rooms: [Vec<Option<Amphipod>>; 4],
    room_len: usize,
}

impl Siderooms {
    fn new(amphipods: Vec<Vec<Amphipod>>) -> Siderooms {
        let room_len = amphipods.first().unwrap().len();
        let mut rooms = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for (i, r) in amphipods.into_iter().enumerate() {
            for a in r.into_iter() {
                rooms[i].push(Some(a));
            }
        }
        Siderooms { rooms, room_len }
    }

    fn settle_idx(&self, a_type: AType) -> Option<usize> {
        let room = &self.rooms[a_type.room_no()];
        let mut cells = room.iter();
        if cells.next().unwrap().is_some() {
            return None;
        }
        let mut idx = 0;
        for (i, cell) in cells.enumerate() {
            match cell {
                None => {
                    idx += 1;
                    if idx != i + 1 {
                        panic!("room is not packed correctly")
                    }
                    continue;
                }
                Some(a) => {
                    if a.is_settled {
                        return Some(idx);
                    } else {
                        return None;
                    }
                }
            }
        }
        Some(idx)
    }

    fn put(&self, amphipod: Amphipod, idx: usize) -> Self {
        let mut rooms = self.rooms.clone();
        let room = &mut rooms[amphipod.typ.room_no()];
        if room[idx].is_some() {
            panic!("can't put amphipod in an occupied room cell")
        }
        room[idx] = Some(amphipod);

        Siderooms {
            rooms,
            room_len: self.room_len,
        }
    }

    fn drop(&self, room_no: usize, idx: usize) -> (Self, Amphipod) {
        let mut rooms = self.rooms.clone();
        let cell = rooms[room_no][idx];

        match cell {
            None => panic!("can't drop from empty cell"),
            Some(a) => {
                rooms[room_no][idx] = None;
                (
                    Siderooms {
                        rooms,
                        room_len: self.room_len,
                    },
                    a,
                )
            }
        }
    }

    fn is_done(&self, i: usize) -> bool {
        for cell in &self.rooms[i] {
            match cell {
                None => return false,
                Some(amphipod) => {
                    if amphipod.typ.room_no() != i {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn are_all_done(&self) -> bool {
        for i in 0..self.rooms.len() {
            if !self.is_done(i) {
                return false;
            }
        }
        true
    }

    fn iter_ready<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.rooms
            .iter()
            .enumerate()
            .map(|(room_idx, room)| {
                for (i, cell) in room.iter().enumerate() {
                    match cell {
                        None => continue,
                        Some(a) => {
                            if a.is_settled {
                                return None;
                            } else {
                                return Some((room_idx, i));
                            }
                        }
                    };
                }
                None
            })
            .filter(Option::is_some)
            .map(Option::unwrap)
    }
}

fn get_min_energy(hallway: Hallway, siderooms: Siderooms, energy: u64, total_min: u64) -> u64 {
    if siderooms.are_all_done() {
        return energy;
    }

    let mut min_energy = u64::MAX;

    for hallway_idx in hallway.iter_occupied() {
        let a_type = hallway.cells[hallway_idx].unwrap().typ;
        let room_idx = match siderooms.settle_idx(a_type) {
            None => continue,
            Some(i) => i,
        };

        let room_entrance = a_type.room_entrance();
        let has_path = hallway.has_path_from_to(hallway_idx, room_entrance);
        if !has_path {
            continue;
        }

        let (new_hallway, mut amphipod) = hallway.drop(hallway_idx);

        let hw_steps = (hallway_idx as i32 - room_entrance as i32).abs() as u64;
        let r_steps = room_idx as u64 + 1;
        let energy_delta = (hw_steps + r_steps) * amphipod.typ.step_energy();
        let new_energy = energy + energy_delta;
        if new_energy > total_min {
            continue;
        }

        amphipod.is_settled = true;

        let new_siderooms = siderooms.put(amphipod, room_idx);

        min_energy = cmp::min(
            min_energy,
            get_min_energy(
                new_hallway,
                new_siderooms,
                new_energy,
                cmp::min(total_min, min_energy),
            ),
        );
    }

    for (room_no, room_idx) in siderooms.iter_ready() {
        let room_entrance = (room_no + 1) * 2;
        for hallway_idx in hallway.paths_from(room_entrance) {
            let (new_siderooms, amphipod) = siderooms.drop(room_no, room_idx);

            let hw_steps = (hallway_idx as i32 - room_entrance as i32).abs() as u64;
            let r_steps = room_idx as u64 + 1;
            let energy_delta = (hw_steps + r_steps) * amphipod.typ.step_energy();
            let new_energy = energy + energy_delta;
            if new_energy > total_min {
                continue;
            }

            let new_hallway = hallway.put(amphipod, hallway_idx);

            min_energy = cmp::min(
                min_energy,
                get_min_energy(
                    new_hallway,
                    new_siderooms,
                    new_energy,
                    cmp::min(total_min, min_energy),
                ),
            );
        }
    }

    min_energy
}

fn get_inputs<'a>(lines: impl Iterator<Item = &'a str> + 'a) -> Vec<Vec<Amphipod>> {
    let mut amphipods = vec![vec![]; 4];
    for line in lines.skip(2) {
        let mut room_idx = 0;
        for c in line.chars() {
            if c == '#' || c == ' ' {
                continue;
            }
            amphipods[room_idx].push(Amphipod {
                typ: AType::from_char(c),
                is_settled: false,
            });
            room_idx += 1;
        }
    }
    for (i, room) in amphipods.iter_mut().enumerate() {
        for a in room.iter_mut().rev() {
            if a.typ.room_no() == i {
                a.is_settled = true;
            } else {
                break;
            }
        }
    }
    amphipods
}

#[allow(dead_code)]
pub fn solve1(text: &str) -> u64 {
    let amphipods = get_inputs(text.lines());
    let hallway = Hallway::new();
    let siderooms = Siderooms::new(amphipods);
    get_min_energy(hallway, siderooms, 0, u64::MAX)
}

#[allow(dead_code)]
pub fn solve2(text: &str) -> u64 {
    let extra_lines = ["  #D#C#B#A#", "  #D#B#A#C#"];
    let amphipods = get_inputs(
        text.lines()
            .take(3)
            .chain(extra_lines)
            .chain(text.lines().skip(3)),
    );
    let hallway = Hallway::new();
    let siderooms = Siderooms::new(amphipods);
    get_min_energy(hallway, siderooms, 0, u64::MAX)
}
