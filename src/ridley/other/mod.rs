mod guard;
mod escape;
mod landing;
// mod air_catch;
mod throw;
mod appeal;

pub fn install(agent: &mut smashline::Agent) {
    guard::install(agent);
    escape::install(agent);
    landing::install(agent);
    // air_catch::install(agent);
    throw::install(agent);
    appeal::install(agent);
}