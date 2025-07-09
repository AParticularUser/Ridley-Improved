mod smashes;
mod aerials;

pub fn install(agent: &mut smashline::Agent) {
    smashes::install(agent);
    aerials::install(agent);
}