use crate::countries::country_routes;
use crate::game::game_routes;
use crate::leagues::league_routes;
use crate::player::player_routes;
use crate::teams::team_routes;
use crate::GameAppData;
use axum::Router;
use axum_extra::routing::SpaRouter;

pub struct ServerRoutes;

impl ServerRoutes {
    pub fn create() -> Router<GameAppData> {
        let app = Router::<GameAppData>::new()
            .merge(country_routes())
            .merge(game_routes())
            .merge(league_routes())
            .merge(team_routes())
            .merge(player_routes())
            .merge(SpaRouter::new("/assets", "assets").index_file("index.html"));

        app
    }
}
