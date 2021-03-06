pub fn get_outliers(mut data_vec: Vec<usize>, is_sorted: bool) -> Option<(Vec<usize>, Vec<usize>)> {
    if data_vec.is_empty() {
        return None;
    }

    if !is_sorted {
        data_vec.sort();
    }

    if let Some((q1_value, _, q3_value)) = get_quartile_values(&data_vec) {
        let interquartile_range = q3_value - q1_value;

        let intermediate_value = 1.5 * interquartile_range;
        let lower_range = q1_value - intermediate_value;
        let upper_range = q3_value + intermediate_value;

        let mut lower_outliers: Vec<usize> = Vec::new();
        let mut upper_outliers: Vec<usize> = Vec::new();

        for data in data_vec {
            if (data as f32) < lower_range {
                lower_outliers.push(data);
            } else if (data as f32) > upper_range {
                upper_outliers.push(data);
            }
        }

        if lower_outliers.is_empty() && upper_outliers.is_empty() {
            return None;
        }

        return Some((lower_outliers, upper_outliers));
    }

    None
}

#[test]
fn get_outliers_empty_data_set() {
    assert_eq!(get_outliers([].to_vec(), true), None);
}

#[test]
fn get_outliers_set_of_one() {
    assert_eq!(get_outliers([30].to_vec(), true), None);
}

#[test]
fn get_outliers_set_of_two() {
    assert_eq!(get_outliers([30, 90].to_vec(), true), None);
}

#[test]
fn get_outliers_none() {
    assert_eq!(get_outliers([1, 2, 4, 10].to_vec(), true), None);
}

#[test]
fn get_outliers_some_1() {
    assert_eq!(
        get_outliers(
            [10, 12, 11, 15, 11, 14, 13, 17, 12, 22, 14, 11].to_vec(),
            false
        ),
        Some(([].to_vec(), [22].to_vec()))
    );
}

#[test]
fn get_outliers_some_2() {
    assert_eq!(
        get_outliers(
            [0, 3, 3, 3, 11, 12, 13, 15, 19, 20, 29, 40, 79].to_vec(),
            false
        ),
        Some(([].to_vec(), [79].to_vec()))
    );
}

fn get_quartile_values(data_vec: &[usize]) -> Option<(f32, f32, f32)> {
    let data_vec_length = data_vec.len();

    if data_vec_length < 2 {
        return None;
    }

    let mut halfway = data_vec_length / 2;

    let q1_value = get_median(&data_vec[0..halfway]);
    let q2_value = get_median(&data_vec);

    if data_vec_length % 2 != 0 {
        halfway += 1;
    }

    let q3_value = get_median(&data_vec[halfway..data_vec_length]);

    Some((q1_value.unwrap(), q2_value.unwrap(), q3_value.unwrap()))
}

#[test]
fn get_quartile_values_empty_set() {
    assert!(get_quartile_values(&[]).is_none());
}

#[test]
fn get_quartile_values_set_of_one() {
    assert!(get_quartile_values(&[10]).is_none());
}

#[test]
fn get_quartile_values_set_of_two() {
    assert_eq!(get_quartile_values(&[10, 12]), Some((10.0, 11.0, 12.0)));
}

#[test]
fn get_quartile_values_set_of_three() {
    assert_eq!(get_quartile_values(&[10, 11, 12]), Some((10.0, 11.0, 12.0)));
}

// [1   2   3   4]   [5   6   7   8]
//        |        |        |
//        Q1       Q2       Q3
#[test]
fn get_quartile_values_even_set_even_halves() {
    assert_eq!(
        get_quartile_values(&[1, 2, 3, 4, 5, 6, 7, 8]),
        Some((2.5, 4.5, 6.5))
    );
}

// [1   2   3]   [4   5   6]
//      |      |      |
//      Q1     Q2     Q3
#[test]
fn get_quartile_values_even_set_odd_halves() {
    assert_eq!(
        get_quartile_values(&[1, 2, 3, 4, 5, 6]),
        Some((2.0, 3.5, 5.0))
    );
}

// [1   2   3   4]   5   [6   7   8   9]
//        |          |          |
//        Q1         Q2         Q3
#[test]
fn get_quartile_values_odd_set_even_halves() {
    assert_eq!(
        get_quartile_values(&[1, 2, 3, 4, 5, 6, 7, 8, 9]),
        Some((2.5, 5.0, 7.5))
    );
}

// [1   2   3   4   5]   6   [7   8   9   10   11]
//          |            |            |
//          Q1           Q2           Q3
#[test]
fn get_quartile_values_odd_set_odd_halves() {
    assert_eq!(
        get_quartile_values(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]).unwrap(),
        (3.0, 6.0, 9.0)
    );
}

fn get_median(data_vec: &[usize]) -> Option<f32> {
    let data_vec_length = data_vec.len();

    if data_vec_length == 0 {
        return None;
    }

    let half_way = data_vec_length / 2;

    if data_vec.len() % 2 == 0 {
        return Some((data_vec[half_way - 1] as f32 + data_vec[half_way] as f32) / 2.0);
    }

    Some(data_vec[half_way] as f32)
}

#[test]
fn get_median_no_elements() {
    assert!(get_median(&[]).is_none());
}

#[test]
fn get_median_one_element() {
    assert!((get_median(&[3]).unwrap() - 3.0).abs() < 0.0001);
}

#[test]
fn get_median_even_set() {
    assert!((get_median(&[1, 2, 3, 4, 5, 6]).unwrap() - 3.5).abs() < 0.0001);
}

#[test]
fn get_median_odd_set() {
    assert!((get_median(&[1, 2, 3, 4, 5]).unwrap() - 3.0).abs() < 0.0001);
}

#[test]
fn get_median_random_numbers_even_set() {
    assert!((get_median(&[1, 11, 34, 66, 209, 353, 1067, 10_453]).unwrap() - 137.5).abs() < 0.0001);
}

#[test]
fn get_median_random_numbers_odd_set() {
    assert!((get_median(&[1, 23, 24, 45, 200, 343, 1001]).unwrap() - 45.0).abs() < 0.0001);
}

pub fn get_average(data_vec: &[usize]) -> f32 {
    if data_vec.is_empty() {
        return 0.0;
    }

    let sum_of_file_name_lengths: usize = data_vec.iter().sum();
    sum_of_file_name_lengths as f32 / data_vec.len() as f32
}

#[test]
fn get_average_no_elements() {
    assert!((get_average(&[])).abs() < 0.0001);
}

#[test]
fn get_average_1() {
    assert!((get_average(&[11, 100, 21, 34]) - 41.5).abs() < 0.0001);
}

#[test]
fn get_average_2() {
    assert!((get_average(&[3, 0, 49, 2000]) - 513.0).abs() < 0.0001);
}
