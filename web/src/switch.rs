use yew_router::prelude::*;

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/blueprint"]
    Blueprint,
    #[to = "/market/bulk"]
    MarketSellBulk,
    #[to = "/market"]
    Market,
}
