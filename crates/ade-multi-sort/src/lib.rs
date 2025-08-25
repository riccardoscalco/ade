pub fn multi_sort<T>(items: &mut [T], metrics: &[Box<dyn Fn(&T) -> i32>]) {
    items.sort_by(|a, b| {
        for metric in metrics {
            let ord = metric(a).cmp(&metric(b));
            if ord != std::cmp::Ordering::Equal {
                return ord;
            }
        }
        std::cmp::Ordering::Equal
    });
}

#[cfg(test)]

mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn test_multi_sort_xy() {
        let mut points = vec![
            Point { x: 2, y: 3 },
            Point { x: 1, y: 5 },
            Point { x: 2, y: 1 },
            Point { x: 1, y: 2 },
        ];

        multi_sort(
            &mut points,
            &[Box::new(|p: &Point| p.x), Box::new(|p: &Point| p.y)],
        );

        let expected = vec![
            Point { x: 1, y: 2 },
            Point { x: 1, y: 5 },
            Point { x: 2, y: 1 },
            Point { x: 2, y: 3 },
        ];

        assert_eq!(points, expected);
    }

    #[test]
    fn test_multi_sort_empty() {
        let mut points: Vec<Point> = Vec::new();
        multi_sort(&mut points, &[Box::new(|p: &Point| p.x)]);
        assert!(points.is_empty());
    }
}
