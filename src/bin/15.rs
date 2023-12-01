use std::collections::{HashSet};
use std::ops::Range;

pub fn part_one(input: &str) -> Option<i32> {
    let sensors = parse(input);
    let mut x_set:HashSet<i32>= HashSet::new();
    sensors.iter().for_each(|sensor|{
        if let Some(positions) = sensor.scan_in_y(2000000){
            positions.iter().for_each(|s|{
                if s.y == 2000000 {
                    x_set.insert(s.x);
                }
            });
        }
    });
    Some(x_set.len() as i32)
}

fn dis(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

pub fn part_two(input: &str) -> Option<i64> {
    let sensors  = parse(input);
    for s in sensors.iter() {
      let points = s.generate_outline();
        for p in points {
            if p.x <0 || p.y <0 || p.x >4_000_000 || p.y > 4_000_000 {
                continue;
            }

            if !p.has_sensor_in_range(&sensors){
                return Some(p.x as i64 * 4000000i64 + p.y as i64);
            }
        }
    }
    None
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos{
    pub fn has_sensor_in_range(&self, sensors: &Vec<Sensor>) ->bool{
        for s in sensors.iter() {
            if self.is_sensor_in_range(s){
                return true;
            }
        }
        return false;
    }
    fn is_sensor_in_range(&self, s: &Sensor) -> bool {
        return ((self.x - s.pos.x).abs() + (self.y - s.pos.y).abs()) <= s.beacon_distance;
    }
}

#[derive(Debug)]
struct Sensor{
    beacon_distance: i32,
    x_bound: Range<i32>,
    y_bound: Range<i32>,
    nearest_beacon: Pos,
    pos: Pos
}

impl Sensor{
    fn plus_one(&self)->i32{
        return self.beacon_distance + 1;
    }

    fn generate_outline(&self) -> Vec<Pos> {
        let mut v:Vec<Pos> = Vec::new();
        for x in -self.plus_one()..=self.plus_one() {
            let yup = self.plus_one() - x.abs();
            if yup == 0 {
                continue;
            }
            let pos1 = Pos { x: self.pos.x + x, y: self.pos.y + yup };
            let pos2 = Pos { x: self.pos.x + x, y: self.pos.y - yup };
            v.push(pos1);
            v.push(pos2);
        }
        return v;
    }

   fn scan_in_y(&self, y:i32) -> Option<Vec<Pos>> {
       if !self.y_bound.contains(&y) {
           return None;
       }
       let mut positions:Vec<Pos> = vec![];
       self.x_bound.clone().for_each(|x|{
           let neighbor = Pos{ x, y };
           if (neighbor.x != self.nearest_beacon.x || neighbor.y != self.nearest_beacon.y) &&
               dis(self.pos.x, self.pos.y,neighbor.x,neighbor.y) <= self.beacon_distance {
               positions.push(neighbor);
           }
       });

       Some(positions)
   }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn parse(input: &str) -> Vec<Sensor>{
    let lines = input.lines();
    let mut sensors:Vec<Sensor> = Vec::default();
    lines.for_each(|line| {
        let split = line.split(' ');
        let sensor_string_x = split.clone().nth(2).unwrap();
        let sensor_x = sensor_string_x
            .split_once('=')
            .unwrap()
            .1
            .replace(',',"")
            .parse::<i32>()
            .unwrap();
        let sensor_string_y = split.clone().nth(3).unwrap();
        let sensor_y = sensor_string_y
            .split_once('=')
            .unwrap()
            .1
            .replace(':',"")
            .parse::<i32>()
            .unwrap();
        let beacon_string_x = split.clone().nth(8).unwrap();
        let beacon_x = beacon_string_x
            .split_once('=')
            .unwrap()
            .1
            .replace(',',"")
            .parse::<i32>()
            .unwrap();
        let beacon_string_y = split.clone().nth(9).unwrap();
        let beacon_y = beacon_string_y
            .split_once('=')
            .unwrap()
            .1
            .replace(',',"")
            .parse::<i32>()
            .unwrap();

        let sensor = Pos {
            x: sensor_x,
            y: sensor_y,
        };
        let beacon = Pos {
            x: beacon_x,
            y: beacon_y,
        };
        let distance = (sensor.x - beacon.x).abs() + (sensor_y - beacon_y).abs();
        let start_x = sensor.x - distance;
        let start_y = sensor.y - distance;
        let end_x = sensor.x + distance;
        let end_y = sensor.y + distance;
        sensors.push(Sensor{
            beacon_distance: distance,
            x_bound: start_x..end_x,
            y_bound: start_y..end_y ,
            nearest_beacon: beacon,
            pos: sensor
        });
    });
    sensors
}
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn outline(){
        let sensor = Sensor{
            beacon_distance: 9,
            x_bound: -1..17,
            y_bound: -2..16,
            nearest_beacon: Pos { x: 2, y: 10 },
            pos: Pos { x: 8, y: 7 },
        };
        let v = sensor.generate_outline();
        assert_eq!(v.len(), 38);
        assert!(v.contains(&Pos { x: 14, y: 11 }))
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(0));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
