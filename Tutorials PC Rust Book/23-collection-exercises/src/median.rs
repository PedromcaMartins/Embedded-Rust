pub fn median(list: &[i32]) -> Option<i32> {
    let  mut list = list.to_owned();
    let len = list.len();
    list.sort_unstable();
    // println!("{:?}", list);

    match len {
        0 => return None, 
        1 => return Some(list[0]), 
        _ => (),
    };

    if len % 2 == 0 {
        let (index1, index2) = {
            let mid = len / 2;
            (mid - 1, mid) // {_, _, _, _} <== get (2, 3)
        };

        let num1 = list.get(index1);
        let num2 = list.get(index2);

        match (num1, num2) {
            (Some(num1), Some(num2)) => Some((num1 + num2) / 2),
            _ => None,
        }
    } else {
        let index = len / 2;
        let med = list.get(index);

        med.copied()
    }
}

#[test]
fn test_median() {
    assert_eq!(median(&[]), None);
    assert_eq!(median(&[1]), Some(1));
    assert_eq!(median(&[1, 3]), Some(2));
    assert_eq!(median(&[1, 5, 4]), Some(4));
}