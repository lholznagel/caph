use crate::utils::FormatNumberComponent;

use serde::Deserialize;
use std::collections::HashMap;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

#[derive(Clone, Debug, Deserialize)]
pub struct ItemInfo {
    description: String,
    group_id: u32,
    id: u32,
    name: String,
    volume: Option<f32>,
}

pub enum Msg {
    LoadBlueprintCost,
    LoadedBlueprintCost(HashMap<u32, u32>),
    LoadedItemNames(Vec<ItemInfo>),
    UpdateSearch(String),
}

#[derive(Debug, Default)]
pub struct State {
    blueprint_id: u32,
    blueprint_cost: HashMap<u32, u32>,
    item_infos: Vec<ItemInfo>,
}

pub struct BlueprintCostComponent {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    state: State,
}

impl BlueprintCostComponent {
    fn fetch_blueprint_cost(&mut self) {
        let request = Request::get(format!("/api/blueprints/{}", self.state.blueprint_id))
            .body(Nothing)
            .expect("Could not build request");

        let callback = self.link.callback(
            |response: Response<Json<Result<HashMap<u32, u32>, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::LoadedBlueprintCost(data.unwrap_or_default())
            },
        );

        let task = FetchService::fetch(request, callback).expect("Failed to start request");
        self.fetch_task = Some(task);
    }

    fn fetch_item_names(&mut self) {
        let ids = self
            .state
            .blueprint_cost
            .clone()
            .into_iter()
            .map(|(x, _)| x)
            .collect::<Vec<u32>>();

        let request = Request::post("/api/items/bulk")
            .body(Json(&ids))
            .expect("Could not build request");

        let callback = self.link.callback(
            |response: Response<Json<Result<Vec<ItemInfo>, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::LoadedItemNames(data.unwrap_or_default())
            },
        );

        let task = FetchService::fetch(request, callback).expect("Failed to start request");
        self.fetch_task = Some(task);
    }

    fn render_blueprint_item(&self, id: u32) -> Html {
        html! {
            <tr>
                <td>{ id }</td>
                <td>{ self.state.item_infos.clone().into_iter().find(|x| x.id == id).unwrap().name }</td>
                <td><FormatNumberComponent number=self.state.blueprint_cost.clone().get(&id).unwrap().to_string()></FormatNumberComponent></td>
            </tr>
        }
    }

    fn render(&self) -> Html {
        if self.state.item_infos.is_empty() {
            return html! { <></> };
        }

        let mut blueprints = self
            .state
            .blueprint_cost
            .clone()
            .into_iter()
            .map(|(x, _)| x)
            .collect::<Vec<u32>>();
        blueprints.sort_by(|a, b| a.cmp(b));

        html! {
            <table class="table">
                <thead>
                    <tr>
                        <th>{ "Item ID" }</th>
                        <th>{ "Name" }</th>
                        <th>{ "Quantity" }</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        blueprints
                            .into_iter()
                            .map(|x| self.render_blueprint_item(x))
                            .collect::<Html>()
                    }
                </tbody>
            </table>
        }
    }
}

impl Component for BlueprintCostComponent {
    type Properties = ();
    type Message = Msg;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch_task: None,
            state: State::default(),
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoadBlueprintCost => {
                self.fetch_blueprint_cost();
                true
            }
            Msg::LoadedBlueprintCost(x) => {
                self.state.blueprint_cost = x;
                self.fetch_item_names();
                false
            }
            Msg::LoadedItemNames(x) => {
                self.state.item_infos = x;
                true
            }
            Msg::UpdateSearch(x) => {
                self.state.blueprint_id = x.parse().unwrap_or_default();
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
                        placeholder="Blueprint ID"
                        value=&self.state.blueprint_id
                        oninput=&self.link.callback(|e: InputData| Msg::UpdateSearch(e.value)) />
                    <div class="input-group-append">
                        <button
                            class="btn btn-primary"
                            type="button"
                            onclick=self.link.callback(|_| Msg::LoadBlueprintCost)>{ "Calculate" }</button>
                    </div>
                    { self.render() }
                </div>
            </div>
        }
    }
}
