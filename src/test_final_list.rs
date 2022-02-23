#[cfg(test)]
mod test_final_list {
    use super::super::build_final_list;

    #[test]
    fn test_build_final_list() {
        // A bit of a weird test, will do something better later
        let a = build_final_list(4, 0, 9, true);
        assert_eq!(a.len(), 4);
        assert!(a[0] >= 0 && a[0] <= 9);
        assert!(a[1] >= 0 && a[1] <= 9);
        assert!(a[2] >= 0 && a[2] <= 9);
        assert!(a[3] >= 0 && a[3] <= 9);
    }
}