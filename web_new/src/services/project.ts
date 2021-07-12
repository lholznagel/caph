import axios from "axios";
import {GROUPS, IBlueprintMaterial, IBlueprintTree} from "./blueprint";
import {formatNumber} from "./formatNumber";
import {NameService} from "./name";

export class ProjectService {
  public static async projectChests(): Promise<IProjectChestOption[]> {
    return [{
      label: 'z_procurer',
      value: 1036590986685,
    }];
  }

  public static async projects(): Promise<IProject[]> {
    return (await axios.get('/api/projects')).data;
  }

  public static async delete(id: string) {
    return (await axios.delete(`/api/projects/${id}`));
  }

  public static async project(id: string): Promise<IProject[]> {
    return (await axios.get(`/api/projects/${id}`)).data;
  }

  public static async projectNew(body: IProject): Promise<string> {
    return await axios.post(`/api/projects`, body);
  }

  public static async cost(id: string): Promise<IProjectCost[]> {
    return (await axios.get(`/api/projects/${id}/cost`)).data;
  }

  public static async manufacture(id: string): Promise<IRequiredProduction[]> {
    return (await axios.get(`/api/projects/${id}/products`))
      .data
      .sort((a: any, b: any) => b.depth - a.depth);
  }

  public static async materials(id: string): Promise<IBlueprintMaterial[]> {
    return (await axios.get(`/api/projects/${id}/materials`))
      .data
      .map((x: IBlueprintMaterial) => {
        return {
          group:    GROUPS[x.mid],
          mid:      x.mid,
          quantity: x.quantity
        }
      })
      .sort((a: any, b: any) => a.mid - b.mid);
  }

  public static async rawMaterials(id: string): Promise<IBlueprintMaterial[]> {
    return (await axios.get(`/api/projects/${id}/materials/raw`))
      .data
      .map((x: IBlueprintMaterial) => {
        return {
          group:    GROUPS[x.mid],
          mid:      x.mid,
          quantity: x.quantity
        }
      })
      .sort((a: any, b: any) => a.mid - b.mid);
  }

  public static async storedMaterials(id: string) {
    return (await axios.get(`/api/projects/${id}/materials/stored`))
      .data
      .map((x: IProjectMaterial) => {
        return {
          group:    GROUPS[x.type_id],
          ...x
        }
      })
      .sort((a: any, b: any) => a.type_id - b.type_id);
  }

  public static async blueprints(id: string): Promise<IProjectBlueprint[]> {
    return (await axios.get(`/api/projects/${id}/blueprints`))
      .data
      .sort((a: any, b: any) => a - b);
  }

  public static async tree(pid: string): Promise<IBlueprintTree[]> {
    let data = (await axios.get(`/api/projects/${pid}/tree`)).data;
    return this.translateTrees(data);
  }

  private static async translateTrees(trees: any[]): Promise<any[]> {
    let translated = [];

    for (let tree of trees) {
      translated.push(await this.translateTree(tree));
    }

    return translated;
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
}

export interface IProjectChestOption {
  label: string;
  value: number;
}

export interface IProject {
  id?:        string;
  name:       string;
  blueprints: IProjectBlueprint[];
  chest:      number;
  system:     number;
}

export interface IProjectBlueprint {
  bpid: number;
  runs: number;
}

export interface IProjectCost {
    material_total_cost:    number;
    facility_bonus:         number;
    facility_bonus_perc:    number;
    facility_tax:           number;
    facility_tax_perc:      number;
    system_cost_index:      number;
    system_cost_index_perc: number;
    production_cost:        number;
    total_cost:             number;
    sell_price:             number;
    materials:              IProjectMaterialCost[];
    bpid:                   number;
}

export interface IProjectMaterialCost {
  mid: number;
  amount_eff: number;
  amount_orig: number;
  cost: number;
}

export interface IProjectMaterial {
  group?:        string;
  item_id:       number;
  location_flag: string;
  location_id:   number;
  quantity:      number;
  type_id:       number;
}

export interface IRequiredProduction {
  pid:       number;
  bpid:      number;
  quantity:  number;
  materials: IRequiredProductionMaterial[];
  depth:     number;
  stored:    number;
}

export interface IRequiredProductionMaterial {
  mid:      number;
  quantity: number;
  stored:   number;
}

export interface IProjectBlueprint {
  bpid:      number;
  stored:    boolean,
  product:   boolean;
  invention: boolean;
  mat_eff?:  number;
  time_eff?: number;
}
