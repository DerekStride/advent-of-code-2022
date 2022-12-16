use std::{str::FromStr, fmt::Display, collections::HashSet};

use anyhow::Result;

const INPUT: &'static str = include_str!("../../inputs/day15.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point { x: i64, y: i64 }

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({},{})", self.x, self.y)
    }
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> usize {
        (self.x - other.x).abs() as usize + (self.y - other.y).abs() as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Report {
    sensor: Point,
    beacon: Point,
}

impl FromStr for Report {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (sensor_str, beacon_str) = s.split_once(":")
            .ok_or(anyhow::anyhow!("invalid report: {s}"))?;

        let (sensor_x, rest) = sensor_str[12..].split_once(",")
            .ok_or(anyhow::anyhow!("invalid sensor: {sensor_str}"))?;
        let (_, sensor_y) = rest.split_once("=")
            .ok_or(anyhow::anyhow!("invalid sensor_y: {rest}"))?;
        let sensor = Point { x: sensor_x.parse()?, y: sensor_y.parse()? };

        let (beacon_x, rest) = beacon_str[24..].split_once(",")
            .ok_or(anyhow::anyhow!("invalid beacon: {beacon_str}"))?;
        let (_, beacon_y) = rest.split_once("=")
            .ok_or(anyhow::anyhow!("invalid beacon_y: {rest}"))?;
        let beacon = Point { x: beacon_x.parse()?, y: beacon_y.parse()? };

        Ok(Self { sensor, beacon })
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sensor: {} beacon: {} ", self.sensor.to_string(), self.beacon.to_string())?;
        writeln!(f, "manhatten_distance: {}", self.manhattan_distance())?;
        Ok(())
    }
}

impl Report {
    fn manhattan_distance(&self) -> usize {
        self.sensor.manhattan_distance(&self.beacon)
    }
}


fn part1(input: &str, y: i64) -> Result<usize> {
    let reports = input.trim().lines()
        .map(|line| line.parse::<Report>())
        .collect::<Result<Vec<_>>>()?;
    println!("reports: {}", reports.len());

    let largest_distance = reports
        .iter()
        .map(|report| report.manhattan_distance())
        .max()
        .unwrap() as i64;

    let beacons = reports
        .iter()
        .map(|report| report.beacon)
        .collect::<Vec<Point>>();

    let (min_x, max_x) = reports
        .iter()
        .map(|report| {
            (std::cmp::min(report.sensor.x, report.beacon.x), std::cmp::max(report.sensor.x, report.beacon.x))
        })
        .fold((i64::MAX, i64::MIN), |(min, max), (min2, max2)| {
            (std::cmp::min(min, min2), std::cmp::max(max, max2))
        });

    let mut covered_points = HashSet::new();
    for x in (min_x - largest_distance)..=(max_x + largest_distance) {
        let point = Point { x, y };

        for report in &reports {
            if report.sensor.manhattan_distance(&point) <= report.manhattan_distance() {
                covered_points.insert(point);
            }
        };
    }

    let covered_at_y = covered_points
        .into_iter()
        .filter(|p| !beacons.contains(p))
        .collect::<Vec<Point>>();

    Ok(covered_at_y.len())
}

fn part2(input: &str, bound: usize) -> Result<usize> {
    let reports = input.trim().lines()
        .map(|line| line.parse::<Report>())
        .collect::<Result<Vec<_>>>()?;

    for x in 0..=bound {
        let mut y = 0;
        'outer: loop {
            if y >= bound { break; }
            let point = Point { x: x as i64, y: y as i64 };

            for Report { sensor, beacon } in &reports {
                let beacon_distance = sensor.manhattan_distance(beacon);
                let distance = sensor.manhattan_distance(&point);

                if distance > beacon_distance { continue; }

                let x_distance = (sensor.x - x as i64).abs() as usize;

                // Skip ahead by the y component of the distance to the next beacon
                y = sensor.y as usize + (beacon_distance - x_distance) + 1;
                continue 'outer;
            }

            return Ok(4000000 * x + y);
        }
    };

    Err(anyhow::anyhow!("no uncovered point found"))
}

fn main() {
    println!("part 1: {}", part1(INPUT, 2000000).unwrap());
    println!("part 2: {}", part2(INPUT,4000000).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::*;

    fn example_input() -> String {
        make_input(
            r###"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
            "###
            )
    }

    #[test]
    fn test_part1() {
        let input = example_input();
        assert_eq!(26, part1(&input, 10).unwrap());
    }

    #[test]
    fn test_part2() {
        let input = example_input();
        assert_eq!(56000011, part2(&input, 20).unwrap());
    }
}
