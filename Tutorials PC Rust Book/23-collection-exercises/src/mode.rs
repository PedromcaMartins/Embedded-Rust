use std::collections::HashMap;

pub fn mode(list: &[i32]) -> Vec<i32> {
    let mut map = HashMap::new();

    for &num in list {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    let max_value = map.values().copied().max().unwrap_or(0);

    map.iter()
        .filter(|(_, &value)| value == max_value)
        .map(|(&key, _)| key )
        .collect()
}

#[test]
fn test_mode() {
    let mut real1 = mode(&[]);
    let expected1 = &[];
    
    let mut real2 = mode(&[6]);
    let expected2 = &[6];

    let mut real3 = mode(&[1, 2, 1, 4, 2]);
    let expected3 = &[1, 2];

    /* -------------------------------------------------------------------------- */

    real1.sort_unstable();
    let real1 = real1.as_slice();
    assert_eq!(real1, expected1);

    real2.sort_unstable();
    let real2 = real2.as_slice();
    assert_eq!(real2, expected2);

    real3.sort_unstable();
    let real3 = real3.as_slice();
    assert_eq!(real3, expected3);
}