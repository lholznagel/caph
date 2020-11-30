use crate::pages::*;
use crate::switch::AppRoute;

use yew::prelude::*;
use yew_router::prelude::*;

pub struct AppComponent;

impl Component for AppComponent {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        AppComponent
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        log::info!("rendered!");
        html! {
            <div class="page-wrapper with-navbar with-sidebar">
                <div class="sticky-alerts"></div>

                <nav class="navbar">
                    <span href="#" class="navbar-brand">
                            { "Caph" }
                    </span>
                </nav>

                <div class="sidebar">
                    <div class="sidebar-menu">
                        <a href="/market" class="sidebar-link">{ "Market" }</a>
                        <a href="/market/bulk" class="sidebar-link">{ "Market sell bulk" }</a>
                        <a href="/blueprint" class="sidebar-link">{ "Blueprint" }</a>
                        <a href="/debug" class="sidebar-link">{ "Debug" }</a>
                    </div>
                </div>

                <div class="content-wrapper">
                    <Router<AppRoute, ()> render = Router::render(Self::switch) />
                </div>
            </div>
        }
    }
}

impl AppComponent {
    fn switch(switch: AppRoute) -> Html {
        match switch {
            AppRoute::Blueprint => html! { <BlueprintCostComponent /> },
            AppRoute::Market => html! { <MarketComponent /> },
            AppRoute::MarketSellBulk => html! { <MarketSellBulkComponent /> },
        }
    }
}
