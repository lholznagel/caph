use serde::Deserialize;
use serde_json::json;
use yew::{
    format::Json,
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

#[derive(Clone, Debug, Deserialize)]
pub struct MarketEntry {
    is_buy_order: bool,
    location_id: u64,
    price: f32,
    system_id: u32,
    type_id: u32,
    volume_remain: u32,
}

pub enum Msg {
    Loaded(Result<Vec<MarketEntry>, anyhow::Error>),
    Search,
}

pub struct State {
    item_name: String,
    entries: Vec<MarketEntry>,
}

pub struct MarketComponent {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    error: Option<String>,
    state: State,
}

impl MarketComponent {
    fn render_market_table_entry(market: &MarketEntry) -> Html {
        html! {
            <tr>
                <td>{ market.location_id } </td>
                <td>{ market.system_id } </td>
                <td>{ market.price } </td>
                <td>{ market.volume_remain } </td>
            </tr>
        }
    }

    fn render_market_table(&self) -> Html {
        if self.state.entries.len() > 0 {
            html! {
                <table class="table">
                    <thead>
                        <tr>
                            <th>{ "Location" }</th>
                            <th>{ "System" }</th>
                            <th>{ "Price" }</th>
                            <th>{ "Volume remaining" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { self
                            .state
                            .entries
                            .iter()
                            .map(Self::render_market_table_entry)
                            .collect::<Html>()
                        }
                    </tbody>
                </table>
            }
        } else {
            html! { <></> }
        }
    }

    fn search(&mut self) {
        let request = json!({
            "ids": [18],
            "onlyBuyOrders": true
        });

        let request = Request::post("/api/market")
            .header("Content-Type", "application/json")
            .body(Json(&request))
            .expect("Could not build request");

        let callback = self.link.callback(
            |response: Response<Json<Result<Vec<MarketEntry>, anyhow::Error>>>| {
                let Json(mut data) = response.into_body();
                if let Ok(mut x) = data {
                    x.sort_by(|a, b| {
                        b.price
                            .partial_cmp(&a.price)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                    data = Ok(x);
                };

                Msg::Loaded(data)
            },
        );

        let task = FetchService::fetch(request, callback).expect("Failed to start request");
        self.fetch_task = Some(task);
    }
}

impl Component for MarketComponent {
    type Properties = ();
    type Message = Msg;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        MarketComponent {
            link,
            fetch_task: None,
            error: None,
            state: State {
                entries: Vec::new(),
                item_name: String::new(),
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Loaded(x) => {
                match x {
                    Ok(entries) => {
                        self.state.entries = entries;
                    }
                    Err(error) => self.error = Some(error.to_string()),
                }
                self.fetch_task = None;
                true
            }
            Msg::Search => {
                self.search();
                false
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="card">
                <div class="input-group">
                    <input
                        class="form-control"
                        placeholder="Item name"
                        value=&self.state.item_name />
                    <div class="input-group-append">
                        <button
                            class="btn btn-primary"
                            type="button"
                            onclick=self.link.callback(|_| Msg::Search)>{ "Search" }</button>
                    </div>
                </div>

                { self.render_market_table() }
            </div>
        }
    }
}
