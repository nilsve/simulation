pub struct KillRegion {
    pub(crate) top_left: (usize, usize),
    pub(crate) bottom_right: (usize, usize),
}

impl KillRegion {
    pub fn is_in_region(&self, coord: &(usize, usize)) -> bool {
        if coord.0 < self.top_left.0 || coord.0 > self.bottom_right.0 {
            return false;
        }

        if coord.1 < self.top_left.1 || coord.1 > self.bottom_right.1 {
            return false;
        }

        true
    }
}

#[test]
fn test_is_in_region() {
    let region = KillRegion {
        top_left: (10, 10),
        bottom_right: (20, 20),
    };

    assert_eq!(region.is_in_region(&(0, 0)), false);
    assert_eq!(region.is_in_region(&(15, 15)), true);
    assert_eq!(region.is_in_region(&(15, 21)), false);
    assert_eq!(region.is_in_region(&(21, 15)), false);
    assert_eq!(region.is_in_region(&(21, 21)), false);
}
