import axios from 'axios';
import {formatNumber} from './formatNumber';
import {IName, NameService} from './name';

export class BlueprintService {
  public static async blueprints(): Promise<IBlueprintOption[]> {
    const ids = (await axios.get('/api/blueprint/')).data;

    let names = await NameService.resolve_bulk(ids);

    const result = [];
    for (const id of ids) {
      result.push({
        label: (names.find((x: IName) => x.id === id) || { name: 'Unknown' }).name,
        value: id
      });
    }

    return result;
  }

  public static async blueprint(bid: number): Promise<IBlueprint> {
    return (await axios.get(`/api/blueprint/${bid}`)).data;
  }

  public static async tree(bid: number): Promise<IBlueprintTree> {
    let data = (await axios.get(`/api/blueprint/${bid}/tree`)).data;
    return this.translateTree(data);
  }

  private static async translateTree(tree: any): Promise<any> {
    const leafs = [];
    for (const leaf of tree.leafs) {
      leafs.push(await this.translateTree(leaf));
    }

    return {
      label:   `${await NameService.resolve(tree.type_id)} (${formatNumber(tree.quantity)})`,
      key:      tree.type_id,
      children: leafs.length > 0 ? leafs : undefined,
    };
  }

  public static async rawMaterials(
    bpids: { bpid: number, runs: number }[]
  ): Promise<IBlueprintMaterial[]> {
    return (await axios.post(`/api/blueprint/material/raw`, bpids))
      .data
      .map((x: IBlueprintMaterial) => {
        return {
          group:    GROUPS[x.mid],
          mid:      x.mid,
          quantity: x.quantity
        }
      });
  }

  public static async manufactureCost(
    iid: number,
    sid: number,
    runs: number,
  ): Promise<IBlueprintCost> {
    return (
      await axios.get(
        `/api/blueprint/manufacture/cost?iid=${iid}&sid=${sid}&runs=${runs}`)
    ).data;
  }

  public static async manufactureMaterial(
    iid: number,
    runs: number,
  ): Promise<IBlueprintMaterialCost[]> {
    return (
      await axios.get(
        `/api/blueprint/manufacture/material?iid=${iid}&runs=${runs}`)
    ).data;
  }
}

export interface IBlueprintOption {
  label: string;
  value: number;
}

export interface IBlueprint {
  bip:           number;
  copy:          IBlueprintAction;
  invention:     IBlueprintAction;
  manufacture:   IBlueprintAction;
  reaction:      IBlueprintAction | null;
  research_mat:  IBlueprintAction;
  research_time: IBlueprintAction;
  limit:         number;
}

export interface IBlueprintAction {
  materials: IBlueprintMaterial[];
  products:  IBlueprintMaterial[];
  skills:    IBlueprintSkill[];
  limit:     number;
}

export interface IBlueprintMaterial {
  mid:      number;
  quantity: number;
  group:    string;
}

export interface IBlueprintSkill {
  level:   number;
  type_id: number;
}

export interface IBlueprintCost {
    material_total_cost:    number,
    facility_bonus:         number,
    facility_bonus_perc:    number,
    facility_tax:           number,
    facility_tax_perc:      number,
    system_cost_index:      number,
    system_cost_index_perc: number,
    production_cost:        number,
    total_cost:             number,
    sell_price:             number,
    materials:              IBlueprintMaterialCost[],
}

export interface IBlueprintMaterialCost {
  mid: number;
  amount_eff: number;
  amount_orig: number;
  cost: number;
}

export interface IBlueprintTree {
  label:    string;
  key:      number;
  children: IBlueprintTree[];
}

export const GROUPS: { [key: number]: string } = {
  34:    'Asteroid',
  35:    'Asteroid',
  36:    'Asteroid',
  37:    'Asteroid',
  38:    'Asteroid',
  39:    'Asteroid',
  40:    'Asteroid',
  2267:  'Planetary Interaction',
  2268:  'Planetary Interaction',
  2270:  'Planetary Interaction',
  2272:  'Planetary Interaction',
  2306:  'Planetary Interaction',
  2307:  'Planetary Interaction',
  2308:  'Planetary Interaction',
  2309:  'Planetary Interaction',
  2310:  'Planetary Interaction',
  3830:  'Blueprint',
  3685:  'Commodity',
  11399: 'Asteroid',
  16272: 'Ice',
  16273: 'Ice',
  16274: 'Ice',
  16275: 'Ice',
  16633: 'Moon',
  16634: 'Moon',
  16635: 'Moon',
  16636: 'Moon',
  16637: 'Moon',
  16638: 'Moon',
  16639: 'Moon',
  16640: 'Moon',
  16641: 'Moon',
  16642: 'Moon',
  16643: 'Moon',
  16644: 'Moon',
  16646: 'Moon',
  16647: 'Moon',
  16648: 'Moon',
  16649: 'Moon',
  16650: 'Moon',
  16652: 'Moon',
  17359: 'Blueprint',
  17887: 'Ice',
  17888: 'Ice',
  17889: 'Ice',
  25589: 'Salvage',
  25598: 'Salvage',
  25599: 'Salvage',
  25600: 'Salvage',
  25601: 'Salvage',
  25606: 'Salvage',
};
