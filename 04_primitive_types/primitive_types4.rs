//primitive_types4

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // up to 4 (4 is exlcuded)
        let nice_slice = &a[1..4];
        assert_eq!([2, 3, 4], nice_slice);

        // upper index can be included by using `=` sign
        let nice_slice = &a[1..=3];
        assert_eq!([2, 3, 4], nice_slice);
    }
}
