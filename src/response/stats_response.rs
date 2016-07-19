
#[derive(Debug)]
struct StatsResponseItem {
    name: String,
    value: String,
}

type StatsResponse = Vec<StatsResponseItem>;
