use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use yew::{
    format::Json,
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

#[derive(Clone, Debug, Deserialize)]
pub struct MarketEntry {
    price: f32,
    system_id: u32,
    type_id: u32,
    volume_remain: u32,
    amount: Option<u32>
}

#[derive(Clone, Debug, Deserialize)]
pub struct ItemEntry {
    id: u32,
    name: String,
}

pub enum Msg {
    UpdatedItems(String),
    ResolvedItems(HashMap<u32, String>),
    FetchedMarket(HashMap<u32, Vec<MarketEntry>>),
    ResolveItems,
}

pub struct State {
    items: String,
    resolved_items: HashMap<u32, String>,
    market_entries: HashMap<u32, Vec<MarketEntry>>
}

pub struct MarketSellBulkComponent {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    state: State,
}

impl MarketSellBulkComponent {
    fn render_table_entry(&self, market: &MarketEntry) -> Html {
        html! {
            <tr>
                <td>{ market.system_id }</td>
                <td>{ "Unknown" }</td>
                <td>{ market.price }</td>
                <td>{ market.volume_remain }</td>
            </tr>
        }
    }

    fn render_table(&self, id: &u32, entries: &Vec<MarketEntry>) -> Html {
        html! {
            <div class="content">
                <h2 class="content-title">
                    { self.state.resolved_items.get(id).unwrap() }
                </h2>
                <p>
                    <table class="table table-striped table-hover">
                        <thead>
                            <tr>
                                <th>{ "System" }</th>
                                <th>{ "Security" }</th>
                                <th>{ "Price" }</th>
                                <th>{ "Volume remaining" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            {
                                entries
                                    .iter()
                                    .map(|x| self.render_table_entry(x))
                                    .collect::<Html>()
                            }
                        </tbody>
                    </table>
                </p>
            </div>
        }
    }

    fn render_item(&self) -> Html {
        if self.state.market_entries.len() > 0 {
            html! {
                { 
                    self
                        .state
                        .market_entries
                        .iter()
                        .map(|(id, entries)| self.render_table(id, entries))
                        .collect::<Html>()
                }
            }
        } else {
            html! { <></> }
        }
    }

    fn resolve_item_names(&mut self, parsed: HashMap<String, u32>) {
        let names = parsed.into_iter().map(|(x, _)| x).collect::<Vec<String>>();

        let request = Request::post("/api/items/search?exact=true")
            .header("Content-Type", "application/json")
            .body(Json(&names))
            .expect("Could not build request");

        let callback = self.link.callback(
            |response: Response<Json<Result<HashMap<String, Vec<ItemEntry>>, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                if let Ok(x) = &data {
                    let mut result = HashMap::new();
                    for (name, entry) in x {
                        result.insert(entry[0].id, name.clone());
                    }
                    return Msg::ResolvedItems(result);
                }
                Msg::ResolvedItems(HashMap::new())
            },
        );

        let task = FetchService::fetch(request, callback).expect("Failed to start request");
        self.fetch_task = Some(task);
    }

    fn fetch_market(&mut self) {
        let ids = self.state.resolved_items.clone().into_iter().map(|(x, _)| x).collect::<Vec<u32>>();
        let request = json!({ "ids": ids, "onlyBuyOrders": true });

        let request = Request::post("/api/market?sort_price=DESC&max_items=5")
            .header("Content-Type", "application/json")
            .body(Json(&request))
            .expect("Could not build request");

        let callback = self.link.callback(
            move |response: Response<Json<Result<HashMap<u32, Vec<MarketEntry>>, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                if let Ok(x) = &data {
                    Msg::FetchedMarket(x.clone())
                } else {
                    Msg::FetchedMarket(HashMap::new())
                }
            },
        );

        let task = FetchService::fetch(request, callback).expect("Failed to start request");
        self.fetch_task = Some(task);
    }

    fn parse_input(&mut self) -> HashMap<String, u32> {
        let mut items = HashMap::new();

        for item in self.state.items.split('\n') {
            let mut item = item.split('\t');

            let name = item.next().unwrap_or_default();
            let amount = item
                .next()
                .unwrap_or_default()
                .parse::<u32>()
                .unwrap_or_default();

            items.insert(name.into(), amount);
        }
        items
    }
}

impl Component for MarketSellBulkComponent {
    type Properties = ();
    type Message = Msg;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        MarketSellBulkComponent {
            link,
            fetch_task: None,
            state: State {
                items: String::new(),
                resolved_items: HashMap::new(),
                market_entries: HashMap::new(),
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchedMarket(x) => {
                self.fetch_task = None;
                self.state.market_entries = x;
                true
            }
            Msg::ResolveItems => {
                self.fetch_task = None;
                let items = self.parse_input();
                self.resolve_item_names(items);
                true
            }
            Msg::ResolvedItems(x) => {
                self.fetch_task = None;
                self.state.resolved_items = x;
                self.fetch_market();
                true
            }
            Msg::UpdatedItems(x) => {
                self.state.items = x;
                self.parse_input();
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="card">
                <textarea
                    class="form-control"
                    placeholder="Insert items to sell"
                    value=&self.state.items
                    oninput=&self.link.callback(|e: InputData| Msg::UpdatedItems(e.value))
                    ></textarea>

                <div class="text-right">
                    <button
                        class="btn btn-primary"
                        type="button"
                        onclick=self.link.callback(|_| Msg::ResolveItems)>{ "Find prices" }
                    </button>
                </div>

                { self.render_item() }
            </div>
        }
    }
}
