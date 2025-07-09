use crate::imports::*;
use crate::ridley::consts::vars::*;


unsafe extern "C" fn ridley_init(agent: &mut L2CFighterCommon) {
    VarModule::off_flag(agent.module_accessor, instance::RIDLEY_FLAG_SPECIAL_LW_IS_SKEWER);
    VarModule::set_int(agent.module_accessor, instance::RIDLEY_INT_SPECIAL_HI_WALL_COUNT, 0);
}
unsafe extern "C" fn ridley_main(agent: &mut L2CFighterCommon) {
    if VarModule::get_int(agent.module_accessor, instance::RIDLEY_INT_SPECIAL_HI_WALL_COUNT) > 0
    && StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
        VarModule::set_int(agent.module_accessor, instance::RIDLEY_INT_SPECIAL_HI_WALL_COUNT, 0);
    }
}


pub mod consts;
mod normals;
mod specials;
mod other;

pub fn install() {
    let agent = &mut smashline::Agent::new("ridley");
    agent.on_start(ridley_init);
    agent.on_line(Main, ridley_main);
    normals::install(agent);
    specials::install(agent);
    other::install(agent);
    agent.install();
}