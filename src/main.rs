mod aggregation;
mod authorization;
mod diff_handler;
mod dtm;
mod dtm_common;
mod emailer;
mod error;
mod hasher;
mod jwt_handler;
mod launch;
mod mapper;
mod repository;
mod router;
mod service;
mod trait_common;

#[cfg(test)]
mod test;

use rocket::launch;

#[launch]
fn rocket() -> _ {
    launch::launch()
}
