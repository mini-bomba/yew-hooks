use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="app">
            <header class="app-header">
                <h1>{ "Yew Hooks" }</h1>
                <div class="hooks">
                    <h2>{ "State Hooks" }</h2>

                    <ul>
                        <li><Link<AppRoute> to={AppRoute::UseToggle} classes="app-link" >{ "use_toggle" }</Link<AppRoute>> { " - tracks state of counterparts." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseBoolToggle} classes="app-link">{ "use_bool_toggle" }</Link<AppRoute>> { " - tracks state of a boolean." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseCounter} classes="app-link">{ "use_counter" }</Link<AppRoute>> { " - tracks state of a number." }</li>
                    </ul>

                    <h2>{ "Lifecycle Hooks" }</h2>

                    <ul>
                        <li><Link<AppRoute> to={AppRoute::UseEffectOnce} classes="app-link" >{ "use_effect_once" }</Link<AppRoute>> { " - a modified use_effect hook that only runs once." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseMount} classes="app-link">{ "use_mount" }</Link<AppRoute>> { " - calls mount callbacks." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseUnmount} classes="app-link">{ "use_unmount" }</Link<AppRoute>> { " - calls unmount callbacks." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseIsFirstMount} classes="app-link">{ "use_is_first_mount" }</Link<AppRoute>> { " - checks if current render is first." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseIsMounted} classes="app-link">{ "use_is_mounted" }</Link<AppRoute>> { " - tracks if component is mounted." }</li>
                    </ul>

                    <h2>{ "Animation Hooks" }</h2>

                    <ul>
                        <li><Link<AppRoute> to={AppRoute::UseTimeout} classes="app-link" >{ "use_timeout" }</Link<AppRoute>> { " - schedules a timeout to invoke callback." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseInterval} classes="app-link">{ "use_interval" }</Link<AppRoute>> { " - schedules an interval to invoke callback." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseUpdate} classes="app-link">{ "use_update" }</Link<AppRoute>> { " - returns a callback, which re-renders component when called." }</li>
                    </ul>
                </div>
            </header>
        </div>
    }
}
