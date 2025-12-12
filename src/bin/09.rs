advent_of_code::solution!(9);

use advent_of_code::maneatingape::iter::*;
use advent_of_code::maneatingape::parse::*;
use advent_of_code::maneatingape::point::*;

fn parse_data(input: &str) -> Vec<Point> {
    input.iter_signed().chunk::<2>().map(|[x, y]| Point::new(x, y)).collect()
}

fn is_on_edge(v_edges: &[(Point, Point)], h_edges: &[(Point, Point)], p: Point) -> bool {
    let v_start = v_edges.partition_point(|(p1, _)| p1.x < p.x);
    if v_edges[v_start..]
        .iter()
        .take_while(|(p1, _)| p1.x == p.x)
        .any(|(p1, p2)| p1.y <= p.y && p.y <= p2.y)
    {
        return true;
    }

    let h_start = h_edges.partition_point(|(p1, _)| p1.y < p.y);
    h_edges[h_start..]
        .iter()
        .take_while(|(p1, _)| p1.y == p.y)
        .any(|(p1, p2)| p1.x <= p.x && p.x <= p2.x)
}

fn is_inside(v_edges: &[(Point, Point)], p: Point) -> bool {
    let start = v_edges.partition_point(|(p1, _)| p1.x <= p.x);
    let c = v_edges[start..].iter().filter(|(p1, p2)| p1.y <= p.y && p.y < p2.y).count();
    c % 2 == 1
}

fn is_inside_or_on_edge(v_edges: &[(Point, Point)], h_edges: &[(Point, Point)], p: Point) -> bool {
    is_on_edge(v_edges, h_edges, p) || is_inside(v_edges, p)
}

fn crosses_horizontal(v_edges: &[(Point, Point)], edge: (Point, Point)) -> bool {
    let y = edge.0.y;
    let min_x = i32::min(edge.0.x, edge.1.x);
    let max_x = i32::max(edge.0.x, edge.1.x);

    let start = v_edges.partition_point(|(p1, _)| p1.x <= min_x);
    v_edges[start..].iter().take_while(|(p1, _)| p1.x < max_x).any(|(p1, p2)| p1.y < y && y < p2.y)
}

fn crosses_vertical(h_edges: &[(Point, Point)], edge: (Point, Point)) -> bool {
    let x = edge.0.x;
    let min_y = i32::min(edge.0.y, edge.1.y);
    let max_y = i32::max(edge.0.y, edge.1.y);

    let start = h_edges.partition_point(|(p1, _)| p1.y <= min_y);
    h_edges[start..].iter().take_while(|(p1, _)| p1.y < max_y).any(|(p1, p2)| p1.x < x && x < p2.x)
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_data(input);

    let mut result = 0;
    for i in 0..data.len() {
        for j in 0..i {
            let dx: u64 = i32::abs_diff(data[i].x, data[j].x) as u64;
            let dy = i32::abs_diff(data[i].y, data[j].y) as u64;
            result = u64::max(result, (dx + 1) * (dy + 1));
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse_data(input);

    let vertices = data;
    let mut vertical_edges = Vec::with_capacity(vertices.len());
    let mut horizontal_edges = Vec::with_capacity(vertices.len());

    for (a, b) in vertices.iter().zip(vertices.iter().cycle().skip(1)) {
        if a.x == b.x {
            let p1 = Point::new(a.x, i32::min(a.y, b.y));
            let p2 = Point::new(a.x, i32::max(a.y, b.y));
            vertical_edges.push((p1, p2));
        } else {
            let p1 = Point::new(i32::min(a.x, b.x), a.y);
            let p2 = Point::new(i32::max(a.x, b.x), a.y);
            horizontal_edges.push((p1, p2));
        }
    }
    vertical_edges.sort_unstable_by_key(|edge| edge.0.x);
    horizontal_edges.sort_unstable_by_key(|edge| edge.0.y);

    let n = vertices.len();
    let mut sorted_results = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in 0..i {
            let dx = i32::abs_diff(vertices[i].x, vertices[j].x) as u64;
            let dy = i32::abs_diff(vertices[i].y, vertices[j].y) as u64;
            let area = (dx + 1) * (dy + 1);
            sorted_results.push((i, j, area));
        }
    }
    sorted_results.sort_unstable_by_key(|x| x.2);

    let result = sorted_results
        .into_iter()
        .rev()
        .find(|(i, j, _)| {
            let p1 = vertices[*i];
            let p2 = vertices[*j];
            let p3 = Point::new(p1.x, p2.y);
            let p4 = Point::new(p2.x, p1.y);

            is_inside_or_on_edge(&vertical_edges, &horizontal_edges, p3)
                && is_inside_or_on_edge(&vertical_edges, &horizontal_edges, p4)
                && !crosses_horizontal(&vertical_edges, (p1, p4))
                && !crosses_vertical(&horizontal_edges, (p4, p2))
                && !crosses_horizontal(&vertical_edges, (p2, p3))
                && !crosses_vertical(&horizontal_edges, (p3, p1))
        })
        .map(|(_, _, area)| area)
        .unwrap();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(24));
    }
}
