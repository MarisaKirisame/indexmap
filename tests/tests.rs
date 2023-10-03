use indexmap::{indexmap, indexset};

#[test]
fn test_sort() {
    let m = indexmap! {
        1 => 2,
        7 => 1,
        2 => 2,
        3 => 3,
    };

    itertools::assert_equal(
        m.sorted_by(|_k1, v1, _k2, v2| v1.cmp(v2)),
        vec![(7, 1), (1, 2), (2, 2), (3, 3)],
    );
}

#[test]
fn test_sort_set() {
    let s = indexset! {
        1,
        7,
        2,
        3,
    };

    itertools::assert_equal(s.sorted_by(|v1, v2| v1.cmp(v2)), vec![1, 2, 3, 7]);
}

#[test]
fn test_meow() {
    let mut m = indexmap! {
        11 => 5,
        12 => 0,
        6 => 13,
        7 => 14,
        2 => 7,
        10 => 0,
        16 => 4,
        15 => 7,
        5 => 9,
        9 => 13
    };
    let total_len = m.len();
    let v: Vec<_> = m.extract_if(|k, v| (*k + *v) % 2 == 0).collect();
    v.iter().for_each(|(k, v)| assert!((k + v) % 2 == 0));
    m.iter().for_each(|(k, v)| assert!((k + v)% 2 == 1));
    assert!(v.len() + m.len() == total_len);
}
