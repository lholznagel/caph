/* tslint:disable:max-classes-per-file max-line-length */
import { formatNumber } from '../../utils/format';

export type Ewar     = 'Tracking disrupter' | 'Target Jamming' | 'Web';
export type ShipType = 'Frigate' | 'Elite Frigate' | 'Elite Cruiser' | 'Destroyer' | 'Cruiser' | 'Battlecruiser' | 'Industrial' | 'Sentry' | 'Structure';
export type Wd       = 'Warp Disrupt';

export class PocketEnemy {
  public ewar?:    Ewar;
  public loot?:    boolean;
  public wd?:      Wd;
  public trigger?: boolean;

  private enemies: Enemy[];

  constructor(
    private countFrom: number,
    private countTo:   number,
    ...enemies:   Enemy[]
  ) { this.enemies = enemies; }

  public getEnemyNames(): string {
    if (this.enemies.length > 0) {
      const factionName: string = this.enemies[0].name.split(' ')[0];
      const combined: string[] = [];

      combined.push(this.enemies[0].name);
      this.enemies
        .slice(1)
        .forEach(x => combined.push(x.name.replace(factionName, '')));

      return combined.join(' / ');
    } else {
      return this.enemies[0].name;
    }
  }

  public getBounty(): string {
    if (this.enemies.length > 0) {
      return this.enemies
        .map(x => x.bounty)
        .map(x => x.toString())
        .map(x => formatNumber(x))
        .join(' / ');
    } else {
      return formatNumber(this.enemies[0].bounty.toString());
    }
  }

  public getCount(): string {
    if (this.countFrom !== this.countTo) {
      return this.countFrom + ' - ' + this.countTo;
    } else {
      return this.countFrom.toString();
    }
  }

  public setEwar(ewar: Ewar): PocketEnemy {
    this.ewar = ewar;
    return this;
  }

  public setLoot(): PocketEnemy {
    this.loot = true;
    return this;
  }

  public setTrigger(): PocketEnemy {
    this.trigger = true;
    return this;
  }

  public getShip(): string {
    return this.enemies[0].ship;
  }

  public setWd(wd: Wd): PocketEnemy {
    this.wd = wd;
    return this;
  }
}

export class Enemy {
  constructor(
    public ship:   ShipType,
    public name:   string,
    public bounty: number,
  ) {  }
}

// ====================================================
// Frigate
// ====================================================
const sf: ShipType = 'Frigate';
export let AngelViper       = new Enemy(sf, 'Angel Viper', 30000);
export let AngelWrbifier    = new Enemy(sf, 'Angel Webifier', 25000);

export let GistiiImpaler    = new Enemy(sf, 'Gistii Impaler', 9000);

export let GuristasKyoukan  = new Enemy(sf, 'Guristas Kyoukan', 30000);
export let GuristasWebifier = new Enemy(sf, 'Guristas Webifier', 25000);

export let MercenaryFighter = new Enemy(sf, 'Mercenary Fighter', 5000);

export let PithiInfiltrator = new Enemy(sf, 'Pithi Infiltrator', 4500);
export let PithiInvader     = new Enemy(sf, 'Pithi Invader', 4875);
export let PithiPlunderer   = new Enemy(sf, 'Pithi Plunderer', 7500);
export let PithiWrecker     = new Enemy(sf, 'Pithi Wrecker', 7875);

// ====================================================
// Cruiser
// ====================================================
const sc: ShipType = 'Cruiser';
export let GistumBreaker      = new Enemy(sc, 'Gistum Breaker', 57188);
export let GistumCenturion    = new Enemy(sc, 'Gistum Centurion', 79688);
export let GistumCrusher      = new Enemy(sc, 'Gistum Crusher', 51563);
export let GistumDefeater     = new Enemy(sc, 'Gistum Defeater', 62813);
export let GistumLiquidator   = new Enemy(sc, 'Gistum Liqzudator', 74063);
export let GistumMarauder     = new Enemy(sc, 'Gistum Marauder', 68438);
export let GistumPhalanx      = new Enemy(sc, 'Gistum Phalanx', 76875);
export let GistumPredator     = new Enemy(sc, 'Gistum Predator', 43125);

export let MercenaryCorporal  = new Enemy(sc, 'Mercenary Corporal', 110000);
export let MercenaryLieutnant = new Enemy(sc, 'Mercenary Lieutnant', 90000);

export let PithumAscriber     = new Enemy(sc, 'Pithum Ascriber', 43125);
export let PithumInferno      = new Enemy(sc, 'Pithum Inferno', 74063);
export let PithumMortifier    = new Enemy(sc, 'Pithum Mortifier', 68438);
export let PithumNullifier    = new Enemy(sc, 'Pithum Nullifier', 62813);
export let PithumSilencer     = new Enemy(sc, 'Pithum Silencer', 37500);

// ====================================================
// Battlecruiser
// ====================================================
const sbc: ShipType = 'Battlecruiser';
export let GistatisLegionnaire = new Enemy(sbc, 'Gistatis Legionnaire', 131250);
export let GistatisPrimus      = new Enemy(sbc, 'Gistatis Primus', 135000);
export let GistatisTribuni     = new Enemy(sbc, 'Gistatis Tribuni', 138750);

export let PithatisAssaulter   = new Enemy(sbc, 'Pithatis Assaulter', 138750);
export let PithatisEnforcer    = new Enemy(sbc, 'Pithatis Enforcer', 135000);
export let PithatisExecutor    = new Enemy(sbc, 'Pithatis Executor', 131250);

// ====================================================
// Destroyer
// ====================================================
const sd: ShipType = 'Destroyer';
export let GistiorDefiler   = new Enemy(sd, 'Gistior Defiler', 13500);
export let GistiorHaunter   = new Enemy(sd, 'Gistior Haunter', 12375);
export let GistiorSeizer    = new Enemy(sd, 'Gistior Seizer', 14625);

export let PithiorRenegade  = new Enemy(sd, 'Pithior Renegade', 12375);
export let PithiorTerrorist = new Enemy(sd, 'Pithior Terrorist', 14625);

// ====================================================
// Industrial
// ====================================================
const si: ShipType = 'Industrial';
export let GuristasPersonnelTransport = new Enemy(si, 'Guristas Personnel Transport', 25000);

// ====================================================
// Sentry
// ====================================================
const ss: ShipType = 'Sentry';
export let AngelLightMissileBattery = new Enemy(ss, 'Angel Light Missile Battery', 25000);
export let HeavyMissileBattery      = new Enemy(ss, 'Heavy Missile Battery', 40000);
export let StatisTower              = new Enemy(ss, 'Statis Tower', 35000);
export let TowerSentryAngelII       = new Enemy(ss, 'Tower Sentry Angel II', 50000);

// ====================================================
// Structure
// ====================================================
const s: ShipType = 'Structure';
export let AuxPowerArray = new Enemy(s, 'Aux Power Array', 0);
export let RepairStation = new Enemy(s, 'Repair Station', 0);

// ====================================================
// Bounty missing
// ====================================================
export let MercenaryCommander = new Enemy(sc, 'Mercenary Commander', 85000);
export let PersonnelTransport = new Enemy(si, 'Personnel Transport', 0);
export let MercenaryMiner = new Enemy(si, 'Mercenary Miner', 25000);
export let MercenaryEliteFighter = new Enemy(si, 'Mercenary Elite Fighter', 90000);

export let Thief              = new Enemy(sf, 'Thief', 0);
export let Blackbird          = new Enemy(sc, 'Blackbird', 0);
export let Moa                = new Enemy(sc, 'Moa', 0);
export let Caracal            = new Enemy(sc, 'Caracal', 0);
export let Rook               = new Enemy('Elite Cruiser', 'Rook', 0);
export let Cerberus           = new Enemy('Elite Cruiser', 'Cerberus', 0);
export let Ferox              = new Enemy('Battlecruiser', 'Ferox', 0);

export let Raider = new Enemy('Frigate', 'Raider', 0);
export let Sunder = new Enemy('Frigate', 'Sunder', 0);
export let Bomber = new Enemy('Cruiser', 'Bomber', 0);
