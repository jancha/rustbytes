// Rust Bytes Challenge Issue #100 Merging Magical Portals

fn merge_portals(portals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    // exit early if empty
    if portals.is_empty() {
        return portals;
    }
    let mut portals = portals.to_vec();
    //Step 1: sort portals
    portals.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged_portals = Vec::new();
    let (mut cur_start, mut cur_end) = *portals.first().unwrap();
    for i in portals.iter().skip(1) {
        if i.0 > cur_end {
            merged_portals.push((cur_start, cur_end));
            cur_start = i.0;
            cur_end = i.1;
        } else if i.1 > cur_end {
            cur_end = i.1;
        }
    }
    // push last pair
    merged_portals.push((cur_start, cur_end));
    merged_portals
}

#[cfg(test)]
mod tests {
    use super::merge_portals;

    #[test]
    fn test_no_portals() {
        assert_eq!(merge_portals(vec![]), vec![]);
    }

    #[test]
    fn test_single_portal() {
        assert_eq!(merge_portals(vec![(5, 10)]), vec![(5, 10)]);
    }

    #[test]
    fn test_no_overlap() {
        let input = vec![(1, 2), (3, 4), (5, 6)];
        let expected = vec![(1, 2), (3, 4), (5, 6)];
        assert_eq!(merge_portals(input), expected);
    }

    #[test]
    fn test_simple_overlap() {
        let input = vec![(1, 3), (2, 4)];
        let expected = vec![(1, 4)];
        assert_eq!(merge_portals(input), expected);
    }

    #[test]
    fn test_touching_edges() {
        let input = vec![(1, 3), (3, 5), (5, 7)];
        let expected = vec![(1, 7)];
        assert_eq!(merge_portals(input), expected);
    }

    #[test]
    fn test_multiple_merges() {
        let input = vec![(6, 8), (1, 5), (2, 4), (7, 9), (10, 12)];
        let expected = vec![(1, 5), (6, 9), (10, 12)];
        assert_eq!(merge_portals(input), expected);
    }

    #[test]
    fn test_all_overlap() {
        let input = vec![(1, 10), (2, 9), (3, 8), (4, 7)];
        let expected = vec![(1, 10)];
        assert_eq!(merge_portals(input), expected);
    }

    #[test]
    fn test_unsorted_input() {
        let input = vec![(5, 6), (1, 3), (2, 4)];
        let expected = vec![(1, 4), (5, 6)];
        assert_eq!(merge_portals(input), expected);
    }
}
