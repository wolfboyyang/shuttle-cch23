use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use country_boundaries::{CountryBoundaries, LatLon, BOUNDARIES_ODBL_360X180};
use dms_coordinates::DMS;
use s2::{cell::Cell, cellid::CellID};

pub fn task() -> Router {
    Router::new()
        .route("/coords/:id", get(get_coords))
        .route("/country/:id", get(get_country))
}

async fn get_coords(Path(id): Path<String>) -> impl IntoResponse {
    let (latitude_angle, longitude_angle) = get_coordinates(id);

    let latitude = DMS::from_ddeg_latitude(latitude_angle);
    let longitude = DMS::from_ddeg_longitude(longitude_angle);

    format!("{} {}", format_dms(latitude), format_dms(longitude))
}

async fn get_country(Path(id): Path<String>) -> impl IntoResponse {
    let (latitude_angle, longitude_angle) = get_coordinates(id);
    let boundaries = CountryBoundaries::from_reader(BOUNDARIES_ODBL_360X180).unwrap();
    let ids = boundaries.ids(LatLon::new(latitude_angle, longitude_angle).unwrap());

    let country = rust_iso3166::from_alpha2(ids.last().unwrap()).unwrap();

    country.name.split(' ').next().unwrap()
}

fn get_coordinates(id: String) -> (f64, f64) {
    let id = u64::from_str_radix(&id, 2).unwrap();
    let point = Cell::from(CellID(id)).center();
    let latitude = point.latitude().deg();
    let longitude = point.longitude().deg();

    (latitude, longitude)
}

fn format_dms(dms: DMS) -> String {
    format!(
        "{}°{}'{:.3}''{}",
        dms.degrees,
        dms.minutes,
        dms.seconds,
        dms.cardinal.unwrap()
    )
}
