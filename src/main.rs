#[derive(Debug)]
struct FilterCondition<T> {
    field: T,
}

impl<T: PartialOrd> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        &self.field < item
    }
}

fn custom_filter<T>(number_list: Vec<T>, filtering: &FilterCondition<T>) -> Vec<T>
where
    T:PartialOrd,
{
    number_list
        .into_iter()
        .filter(|item| filtering.is_match(item))
        .collect::<Vec<T>>()
}

fn main() {
    let number_list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let filtering = FilterCondition { field: 3 };
    let filtered_result = custom_filter(number_list, &filtering);
    println!("{:?}", filtered_result);
}