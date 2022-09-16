# Ridley-Improved
This mod aims to help make Ridley feel more like he does in Metroid by adding a new down-air and aerial down-special, pogo tail stab, along with some stat tweaks, and other technical changes.
## Changes

#### Increased max jumps 3->4
#### Increased weight 107->120
#### Added screen shake effects to aerial-landings, heavy-landing, and down-smash

### Down-Specail
- increased bonus shield-damage: -37->-20
- increased shield-stun multiplier: 0.2->0.4
- if an opponent is airborn when released they will be put into the foot-stooled state intead of being launched
#### Added new aerial down-special: pogo stab
- if the tail hits the ground or an opponent, Ridley will bounce upward relative to the distance from the ground
- the tip of the tail does more damage and spikes
### Added z-air/air-grab
- pressing grab in the air will make Ridley do the aerial version of grounded down-special
### Down-Air
- Ridley now does a tail swat instead of a "stall and fall"
- the tip of the tail does more damage and knockback
### Neutral-Special
- Reduced start up
- reduced end lag of the failure state (faf:56->30)
- added intangablity to "head" and "mouth1" hurt-boxes while charging
- increased size of weak-point hurt-box (1->1.25)
- holding attack after fully charging plasma-breath will make Ridley release a large explosion (like f-smash) instead of shooting fire balls
### Up-Special
- Reduced start up
- landing down now has cancel-frames (faf:38->29)
- landing forward now has cancel-frames (faf:34->28)
- added hit-box to wall and ceiling bonk
- bouncing off a wall no longer puts Ridley into special-fall, but each consecutive bounce without touching the ground only goes half as high
### Side-Special
- landing lag is now dependant on how far into the move Ridley was before landing (the later Ridley lands, the less landing-lag)
- increased size of grab-box to better match animation/effects
- holding attack/special durring the drag will make Ridley do his ledge throw early
- decreased ledge-throw kill power: kbg:90->55, bkb:75->85
### Taunts
- Ridley extends his neck further up during down-taunt
- added wind-boxes to up-taunt
### Throws
Up-throw now kills before down-throw
- up-throw: increased kbg: 120->125
- down-throw: decressed kbg: 115->90, increased bkb: 40->55
