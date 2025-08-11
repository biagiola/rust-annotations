// Lecture: Filter_map method
// The filter_map method both filters and transforms a subset
// of elements from an iterator.

fn main() {
    let stocks = ["nvda", "", "aapl", "msft", "goog"];

    let capitalized_stocks = stocks
        .iter()
        .filter(|stock| !stock.is_empty())
        .map(|stock| stock.to_uppercase())
        .collect::<Vec<String>>();
    println!("{capitalized_stocks:?}");

    let capitalized_stocks = stocks
        .iter()
        .filter_map(|stock| {
            if stock.is_empty() {
                None
            } else {
                Some(stock.to_uppercase())
            }
        })
        .collect::<Vec<String>>();
    println!("{capitalized_stocks:?}");
}