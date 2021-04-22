/* tslint:disable:max-classes-per-file */
import * as e from './enemies';
import {PocketEnemy} from './enemies';

export type Damage = 'EM' | 'Thermal' | 'Kinetic' | 'Explosion';
export type Level  = 'LEVEL_1' | 'LEVEL_2' | 'LEVEL_3' | 'LEVEL_4' | 'LEVEL_5';
export type Typ    = 'MINING' | 'SECURITY';

export class Mission {
  public blitz:         string = '';
  public dmgDeal:     Damage[] = [];
  public dmgResist:   Damage[] = [];
  public eveSurvival:   string;
  public eveUni:        string;
  public info:          string = '';
  public key:           string;
  public pockets:     Pocket[] = [];

  constructor(
    public name:  string,
    public level: Level,
    public typ:   Typ,
  ) {
    this.key = name
      .toUpperCase()
      .replaceAll(' ', '_')
      .replaceAll('(', '')
      .replaceAll(')', '');
    this.eveUni = `https://wiki.eveuniversity.org/` +
                    name.replaceAll(' ', '_') +
                    `_(${level.replace('EVEL', 'evel')})`;
    this.eveSurvival = `https://eve-survival.org/wikka.php?wakka=` +
                        name
                          .replaceAll(' ', '')
                          .replaceAll('-', '')
                          .replaceAll('(GuristasPirates)', '') +
                        level.replace('LEVEL_', '') +
                        (name.indexOf('Guristas') > -1 ? 'gu' : '');
  }

  public setDmgDeal(...dmg: Damage[]): Mission {
    this.dmgDeal = dmg;
    return this;
  }

  public setDmgResist(...dmg: Damage[]): Mission {
    this.dmgResist = dmg;
    return this;
  }

  public setInfo(info: string): Mission {
    this.info = info;
    return this;
  }

  public setBlitz(blitz: string): Mission {
    this.blitz = blitz;
    return this;
  }

  public setPockets(...pocket: Pocket[]): Mission {
    this.pockets = pocket;
    return this;
  }
}

export class Pocket {
  public groups: Group[] = [];

  constructor(
    public name: string,
    public note: string,
    ...mgroups:   Group[]
  ) {
    this.groups = mgroups;
  }
}

export class Group {
  public enemies: PocketEnemy[] = [];

  constructor(
    public name: string,
    public note: string,
    ...menemies: PocketEnemy[]
  ) {
    this.enemies = menemies;
  }
}

export interface IMissionBriefing {
  info:    string;
  blitz:   string;
  pockets: Pocket[];
}

export let missions: Mission[] = [
  new Mission(
      'Unauthorized Military Presence (Guristas Pirates)',
      'LEVEL_3',
      'SECURITY'
    )
    .setBlitz('Kill Group 3, 3b, 4, loot Transport wreck')
    .setDmgDeal('Kinetic', 'Thermal')
    .setDmgResist('Kinetic', 'Thermal')
    .setInfo('The goods are in the Warehouse which is a lootable container')
    .setPockets(
      new Pocket('Pocket 1', 'Will aggro when approaching warpgate',
        new Group('Group 1', '20-30km - aggro on approach to gate',
          new PocketEnemy(4, 4, e.PithiPlunderer, e.PithiWrecker)
            .setEwar('Target Jamming')
        )
      ),
      new Pocket('Pocket 2', 'No aggro on warp in',
        new Group('Group 1', '30-40km',
          new PocketEnemy(4, 4, e.PithiPlunderer, e.PithiWrecker)
            .setEwar('Target Jamming').setTrigger(),
          new PocketEnemy(4, 4, e.PithumSilencer, e.PithumInferno)
        ),
        new Group('Group 2', '70-80km',
          new PocketEnemy(4, 4, e.PithiPlunderer, e.PithiWrecker)
            .setEwar('Target Jamming'),
          new PocketEnemy(1, 1, e.PithumSilencer, e.PithumInferno)
            .setTrigger()
        ),
        new Group('Group 3', '90km',
          new PocketEnemy(4, 4, e.PithiPlunderer, e.PithiWrecker)
            .setEwar('Target Jamming').setTrigger(),
          new PocketEnemy(1, 1, e.GuristasPersonnelTransport)
            .setTrigger()
        ),
        new Group('Group 1 Reinforcement', '90km',
          new PocketEnemy(1, 1, e.PithiPlunderer, e.PithiWrecker),
          new PocketEnemy(1, 1, e.PithumInferno)
            .setEwar('Target Jamming')
        ),
        new Group('Group 2 Reinforcement', '90km',
          new PocketEnemy(3, 3, e.PithumInferno)
            .setEwar('Target Jamming'),
        ),
        new Group('Group 3 Reinforcement', '70-80km',
          new PocketEnemy(3, 3, e.PithumSilencer, e.PithumAscriber)
        )
      )
    ),
  new Mission('Seek and Destroy (Guristas Pirates)', 'LEVEL_3', 'SECURITY')
    .setBlitz('Kill Group 2')
    .setDmgDeal('Kinetic', 'Thermal')
    .setDmgResist('Kinetic', 'Thermal')
    .setPockets(
      new Pocket('Pocket 1', '',
        new Group('Group 1', '40-50km - auto-aggro',
          new PocketEnemy(4, 4, e.PithiWrecker, e.PithiPlunderer),
          new PocketEnemy(2, 2, e.PithumMortifier, e.PithumInferno)
        ),
        new Group('Group 2', '100-110km',
          new PocketEnemy(4, 4, e.PithiPlunderer, e.PithiWrecker),
          new PocketEnemy(1, 1, e.PithatisAssaulter)
        )
      )
    ),
  new Mission('Stop the Thief (Guristas Pirates)', 'LEVEL_3', 'SECURITY')
    .setDmgDeal('Kinetic', 'Thermal')
    .setDmgResist('Kinetic', 'Thermal')
    .setBlitz('Kill the Thief, report shows up in cargo')
    .setPockets(
      new Pocket('Pocket 1', '',
        new Group('Group 1', '20-30km - auto aggro',
          new PocketEnemy(1, 1, e.Thief),
          new PocketEnemy(5, 5, e.MercenaryCommander)
        )
      )
    ),
  new Mission('Retribution (Guristas Pirates)', 'LEVEL_3', 'SECURITY')
    .setBlitz('Destroy outpost')
    .setDmgDeal('Kinetic', 'Thermal')
    .setDmgResist('Kinetic', 'Thermal')
    .setInfo('The small armory drops ammo')
    .setPockets(
      new Pocket('Pocket 1', 'No aggro on warp in',
        new Group('Group 1', '25-30km - Closest Rock formation',
          // Unsure what Pithum Killer and Pithum Murder is
          new PocketEnemy(4, 4, e.PithumAscriber, e.PithumSilencer)
            .setEwar('Target Jamming')
        ),
        new Group('Group 2', '30-40km - Slave Worker Facility',
          new PocketEnemy(5, 5, e.PithumAscriber, e.PithumSilencer)
        ),
        new Group('Group 3', '35-40km - Spaceshuttle Wreck',
          new PocketEnemy(3, 3, e.PithumInferno, e.PithumMortifier)
        ),
        new Group('Group 4', '65-75km - Furthest Rocket Formation',
          new PocketEnemy(3, 3, e.GuristasKyoukan, e.GuristasWebifier)
            .setEwar('Target Jamming').setWd('Warp Disrupt'),
          new PocketEnemy(1, 1, e.PithatisEnforcer)
        )
      )
    ),
  new Mission('Smuggler Interception (Guristas Pirates)', 'LEVEL_3', 'SECURITY')
    .setDmgDeal('Kinetic')
    .setDmgResist('Kinetic', 'Thermal')
    .setPockets(
      new Pocket('Pocket 1', 'No NPC, warp gate only'),
      new Pocket('Pocket 2', '',
        new Group('Group 1', '40-45km - no-auto-aggro',
          new PocketEnemy(4, 4, e.PithiWrecker, e.PithiPlunderer)
        ),
        new Group('Group 2', '30-35km - no-auto-aggro',
          new PocketEnemy(5, 6, e.PithiPlunderer, e.PithiInfiltrator, e.PithiInvader),
          new PocketEnemy(1, 1, e.PithumMortifier, e.PithumInferno)
            .setTrigger()
        ),
        new Group('Reinforcement 1', '40-45km - no-auto-aggro',
          new PocketEnemy(3, 3, e.PithumSilencer, e.PithumAscriber)
            .setTrigger()
        ),
        new Group('Reinforcement 2', '37-55km - auto-aggro',
          new PocketEnemy(4, 4,
              e.PithiInfiltrator, e.PithiInvader,
              e.PithiWrecker, e.PithiPlunderer),
          new PocketEnemy(2, 2, e.PithumSilencer, e.PithumAscriber)
            .setTrigger()
        ),
        new Group('Reinforcement 3', 'Does not always spawn',
          new PocketEnemy(2, 2, e.PithumMortifier, e.PithumInferno)
        )
      ),
      new Pocket('Pocket 3', '',
        new Group('Group 1', '15km - auto-aggro',
          new PocketEnemy(2, 2, e.PersonnelTransport)
            .setLoot(),
          new PocketEnemy(1, 1, e.PithatisEnforcer, e.PithatisExecutor, e.PithatisAssaulter)
            .setTrigger()
        ),
        new Group('Reinforcement 1', '20-25km - auto-aggro',
          new PocketEnemy(5, 5, e.PithiInfiltrator, e.PithiInvader)
        ),
        new Group('Reinforcement 2', '15-20km - auto-aggro',
          new PocketEnemy(5, 5, e.PithumAscriber, e.PithumSilencer)
            .setTrigger()
        ),
        new Group('Reinforcement 3', '30km - auto-aggro',
          new PocketEnemy(2, 2, e.PithumMortifier, e.PithumInferno)
            .setTrigger()
        ),
        new Group('Reinforcement 4', '30km - auto-aggro',
          new PocketEnemy(4, 4, e.PithiPlunderer, e.PithiWrecker)
            .setTrigger()
        ),
        new Group('Reinforcement 5', 'Does not always spawn',
          new PocketEnemy(3, 5, e.PithumMortifier, e.PithumInferno)
        )
      )
    ),
  new Mission('Break Their Will (Gurista Pirates)', 'LEVEL_3', 'SECURITY')
    .setBlitz('Destroy Report Station, warp out')
    .setDmgDeal('Kinetic', 'Thermal')
    .setDmgResist('Kinetic', 'Thermal')
    .setInfo('Repair station repairs itself and rats. Destroy the Aux Power Array disables the Repair Station repair function')
    .setPockets(
      new Pocket('Pocket 1', 'At warp-in there are no rats. Wave 1 spawns when the repait station is hit. Wave 2 is spawned when the Repair station is destroyed. Auto-aggro on waves spawning',
        new Group('Group 1', 'On attack on repair station',
          // Pithior Nihilst
          new PocketEnemy(6, 7, e.PithiorTerrorist, e.PithiorRenegade),
          // Pithum Annihilator / Killer
          new PocketEnemy(5, 6, e.PithumSilencer, e.PithumNullifier)
            .setEwar('Target Jamming'),
          new PocketEnemy(1, 1, e.StatisTower)
            .setEwar('Web'),
          new PocketEnemy(3, 3, e.HeavyMissileBattery)
        ),
        new Group('Group 2', 'On destroy Repair station',
          new PocketEnemy(1, 1, e.MercenaryFighter),
          new PocketEnemy(4, 4, e.MercenaryCommander, e.MercenaryLieutnant, e.MercenaryCorporal)
        )
      )
    ),
  new Mission('Cut-Throat Competition', 'LEVEL_3', 'SECURITY')
    .setBlitz('Destroy ships first wave (Blackbirds only, not the Moa) and destroy repair autpost\nif second wave is triggered continue focus on Blackbirds until mission completes')
    .setDmgDeal('Kinetic', 'Thermal')
    .setDmgResist('Kinetic', 'Thermal')
    .setInfo('Use two Sensor Boosters with ECCM scripts to reduce chance of being jamed.\nKill the Recon Ship (Jammers) before triggering the third wave')
    .setPockets(
      new Pocket('Warp in', '',
        new Group('Group 1', '',
          new PocketEnemy(5, 5, e.Blackbird)
            .setEwar('Target Jamming')
        )
      ),
      new Pocket('Pocket 1', 'Either the Moa or the last ship is trogger for the second wave\nThe Moa has been observed to not spawn\nWhen you first warp in. if there is no Moa, the trigger will be one of the two ships that are a different type from the rest. Usually it is the two that are the first to aggro.\nThere are at least 3 subsequent waves, but they vary in composition order. Consider the following as examples of what you might encounter',
        new Group('Wave 1', 'The recon ships jam, a lot. The last Caracal triggers the third wave. Kill the jamming ships before triggering the third wave',
          new PocketEnemy(1, 1, e.Blackbird)
            .setEwar('Target Jamming'),
          new PocketEnemy(1, 1, e.Moa)
        ),
        new Group('Wave 2', 'The last BC or the last ship triggers the next wave',
          new PocketEnemy(5, 5, e.Caracal)
            .setTrigger(),
          new PocketEnemy(2, 2, e.Rook)
            .setEwar('Target Jamming')
        ),
        new Group('Wave 3', '',
          new PocketEnemy(2, 2, e.Cerberus),
          new PocketEnemy(5, 5, e.Ferox)
            .setTrigger()
        ),
        new Group('Wave 3', '',
          new PocketEnemy(5, 5, e.Ferox)
        )
      )
    ),
  new Mission('New Fronties - Raw Materials', 'LEVEL_3', 'SECURITY')
    .setBlitz('Mine the Green Arisite while tanking')
    .setDmgDeal('Kinetic', 'Thermal')
    .setDmgResist('Kinetic', 'Thermal')
    .setPockets(
      new Pocket('Warp-in', '',
        new Group('Initial Group', 'Miners will warp out after some time',
          new PocketEnemy(3, 3, e.MercenaryMiner)
            .setLoot(),
          new PocketEnemy(3, 3, e.MercenaryEliteFighter)
            .setTrigger(),
          new PocketEnemy(2, 2, e.MercenaryLieutnant)
            .setTrigger()
        ),
        new Group('Wave 1', 'On Group 1 Frigate Destruction',
          new PocketEnemy(4, 4, e.MercenaryCommander)
            .setTrigger()
        ),
        new Group('Wave 2', 'On wave 1 destruction',
          new PocketEnemy(5, 5, e.MercenaryFighter)
        ),
        new Group('Wave 3', 'On Group 1 Cruiser destruction',
          new PocketEnemy(3, 3, e.MercenaryEliteFighter)
            .setTrigger(),
          new PocketEnemy(2, 2, e.MercenaryLieutnant)
            .setTrigger()
        ),
        new Group('Wave 4', 'On wave 3 Frigate destruction',
          new PocketEnemy(4, 4, e.MercenaryCommander)
        ),
        new Group('Wave 5', 'On wave 3 cruiser destruction',
          new PocketEnemy(4, 4, e.MercenaryFighter),
          new PocketEnemy(2, 2, e.MercenaryCommander)
        )
      )
    ),
  new Mission('New Frontiers - Mad Scientist', 'LEVEL_3', 'SECURITY')
    .setBlitz('Go straight through the gate, blow up Professor Delainens Lab and loot the cargo container spawned\nDestroy Statis/Energy Neutralizer as needed')
    .setDmgDeal('Explosion', 'Kinetic', 'Thermal')
    .setDmgResist('EM', 'Thermal')
    .setPockets(
      new Pocket('Warp-In', 'All enemies will aggro immediately',
        new Group('Initial Group', '',
          new PocketEnemy(11, 12, e.Raider, e.Sunder)
        )
      )
    )
];

