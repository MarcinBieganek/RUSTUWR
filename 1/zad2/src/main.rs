fn count_red_beads(n: u32) -> u32 {
    if n < 2 {
        0
    } else {
        (n - 1) * 2
    }
}

mod tests {
    use super::count_red_beads;

    #[test]
    fn test() {
        assert_eq!(count_red_beads(0), 0);
        assert_eq!(count_red_beads(1), 0);
        assert_eq!(count_red_beads(2), 2);
        assert_eq!(count_red_beads(3), 4);
        assert_eq!(count_red_beads(5), 8);
    }
}