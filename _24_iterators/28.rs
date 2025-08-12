// Lecture: Filter_map method
// The filter_map method both filters and transforms a subset
// of elements from an iterator.

fn main() {
    let stocks = ["nvda", "", "aapl", "msft", "", "goog"];

    // filter and map separated
    let capitalized_stocks: Vec<String> = stocks
        .iter()
        .filter(|stock| !stock.is_empty())
        .map(|stock| stock.to_uppercase())
        .collect();
    println!("{capitalized_stocks:?}");

    // filter_map approach
    let capitalized_stocks: Vec<String> = stocks
        .iter()
        .filter_map(|stock| {
            if stock.is_empty() {
                None
            } else {
                Some(stock.to_uppercase())
            }
        })
        .collect();
    println!("{capitalized_stocks:?}");
}