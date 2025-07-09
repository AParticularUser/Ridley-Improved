
mod special_n;
mod special_s;
mod special_hi;
mod special_lw;
mod skewer;

pub fn install(agent: &mut smashline::Agent) {
    special_n::install(agent);
    special_s::install(agent);
    special_hi::install(agent);
    special_lw::install(agent);
    skewer::install(agent);
}