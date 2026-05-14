### Game modes:
- Supports Multiplayer PvP (Team Deathmatch, etc.)
- Supports Multiplayer PvE (Waves Survival, Operation Scenario, etc.)
- Supports Multiplayer Open World

### Entities and relations:
## Focus on Ships and combat aspects:
- A PlayerShip has a certain ShipModel (=A ship has a model)
- A ShipModel has a ShipSize defined by its ShipRole
- ShipModels are listed in the store, and are immutables entites. The user does only mutate his PlayerShips.
- Each ShipModel has a price. User can buy a new PlayerShip of the ShipModel he likes for that price.
- PlayerShips are owned forever by User.
- ShipModels all have a Shield, an Armor and some Energy.
    - Energy continuously regenerates over time.
    - In game, a PlayerShip is destroyed when its Armor reaches zero.
- 3 damage types exist:
    - Electromagnetic: powerful against Shields, usually weak against Armors
    - Kinetic: powerful against Armor, usually weak against Shields
    - Thermic: Balanced power over Armor and Shields
- ShipModels have 1 ShipSize over: Frigate (big), Fighter (medium), Interceptor (small)
    - Frigates can have ShipRole: Engineer (= healer), Long Range (sniper), or Guard (Tank)
    - Fighters can have ShipRole: Tacklers (Can go invisible), GunShip, or Command (Buff)
    - Interceptors can have ShipRole: Cover Ops (like ninja), Recon (Detection capabilities), or ECM (Electronic Counter Measure)
- Each ShipModel has a ShipRole have a SpecialCombatModule type, which can be activated with a special key. The SpecialCombatModule occupies a dedicated 5th slot, separate from the 4 Active Combat Module slots:
    - Engineers have drones (heals Shield on key pressed)
    - Long Ranges have sniper Weapon (toggle on / off on key pressed, then shoot like a regular Weapon)
    - Guard have phasic Shield (toggle improved resistance over Thermic / Electromagnetic / Kinetic on key pressed)
    - Tacklers have Cloak (get invisible on key pressed. That gets turned off when taking damage)
    - GunShips have Overclock (boost Weapons and motors on key pressed)
    - Commands have Command Shield (Activate an additionnal Shield on key presed, which depletes Energy instead of Shield)
    - Cover Ops have Plasma Web (Inflicts Damage On Time to the target on key pressed)
    - Recons have Hyper-Propulsion (Warps far away on key pressed)
    - ECMs have Electromagnetic surge (Freeze every Ship around for a small while)
- Ships can equip PassiveCombatModules: (hull enforcement, speed boosters, etc.) which change Ships stat from their ShipModel's original
- PlayerShips are equiped with EquipablesItems which are:
    - n PassiveCombatModules
    - 4 ActiveCombatModules
    - 1 Weapon
    - 1 MissileLauncher
    - The SpecialCombatModule is hard-build and proper the ShipRole. PlayerShips of any ShipModel inherit of the SpecialCombatModule of their ShipRole.
- Each ShipModel has its own set of PassiveCombatModules (example: 1 for the Shield, 2 for the Armor, etc.)
- Each Passive Combat Module has a type (Shield, Armor, Capacitor, Motor, Computer), and each are different to provide different effects
    - Even if PassiveCombatModules are intended to enforce the Ship part they are for (ex: augment Armor HP for armor typed Passive Module), they may act differently
        - ex: "Light armor" Armor Module would increase speed and lower Armor HP
- Ships can equip 4 ActiveCombatModules, which can be specific to their ShipRoles (Healer module, Shield Boost, etc.)
- ActiveCombatModules consume Energy on activation, and have cooldown
- ActiveCombatModules have seevral possible activation flow:
    - "One Shot": Push activates => provide effect, consumes a finite energy amount, and wait for cooldown
    - "Ongoing": Push toggle on/off => "on" provides effect while continuosly consuming energy. May have cooldown before going "on" again
- Ships can equip exactly 1 Weapon proper to their size (Weapons for Frigates, for Fighters, or for Interceptors). Multiple weapon types exist per size class (e.g., different fire rates, damage per shot, heat generation).
- Ship movement uses arcade-style physics (speed cap, drag, instant acceleration) rather than Newtonian simulation.
- Weapons overload if used for a long period of time, and require to cool down.
- Overheating a Weapon adds up more time for cooling, so it's better to avoid overheating to optimize shooting in long periods
- Ships can shoot Missiles from exactly 1 Missile launcher. Missiles have distinct flight and damage behavior from weapons.

## User Environment:
# Build time: The User build his Ships set
- The User can buy Ships for himself by picking the Ship Model he wants inside the Ships Arborescence
- The User can equip on each ship: PassiveCombatModules (within the ship model's fixed slot layout), up to 4 ActiveCombatModules, 1 Weapon, and 1 Missile launcher.
- The User can equip up to 4 Ships in his Hangar, leaving the other Ships unused

# Play time: The User Participates into a Game instance
- PvP & PvE: The User can pick for a ship of his Hangar to fight with. Respawning (if allowed by the game mode) may allow him to pick another Ship from his Hangar
- Open World: The user comes out of his current Space Station with his currently selected Ship
