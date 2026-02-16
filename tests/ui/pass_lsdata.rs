use wdpe::WdLsData;

#[derive(WdLsData)]
#[allow(unused)]
pub struct SimpleLSData {
    #[wd_lsdata(index = "0")]
    name: Option<String>,
    #[wd_lsdata(index = "1")]
    enabled: Option<bool>,
    #[wd_lsdata(index = "2")]
    count: Option<i32>,
}

fn main() {
    let data = SimpleLSData::default();
    assert!(data.name().is_none());
    assert!(data.enabled().is_none());
    assert!(data.count().is_none());
}
