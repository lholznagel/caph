import { BudgetId, CharacterId, ItemGroup, SystemId, TypeId, Uuid } from "@/utils";
import axios from "axios";
import { IBlueprint, IBuildstep, IDependency, IMaterial, IMember, IProject, IProjectMarket, IRequiredMaterial, IStorageEntry, Service } from "@/project/service";

// Base path that is used for all requests
const BASE_PATH: string    = '/api/v1/projects';
const PROJECT_PATH: string = '/api/v2/projects';

export class Project {
  private load_promise: Promise<void> | null = null;

  public busy: boolean = false;

  public info: IProject = <IProject>{  };
  public raw: IDependency[] = [];
  public buildsteps: IBuildstep = <IBuildstep>{  };
  public stored: IMaterial[] = [];
  public members: IMember[] = [];
  public budget_entries: IBudgetEntry[] = [];
  public market: IAppraisal = <IAppraisal>{  };

  public blueprints: IBlueprintEntry[] = [];

  constructor(
    public pid: ProjectId
  ) {
    this.load_promise = this.load();
  }

  // Loads all information about the project from the server
  public async init(): Promise<void> {
    return this.load_promise
      ? this.load_promise
      : new Promise<void>((r, _) => r());
  }

  public stored_products(): IMaterial[] {
    let result: IMaterial[] = [];

    for (let product of this.info.products) {
      let material = this.stored
        .find(x => x.type_id === product.type_id);

      if (!material) {
        result.push({
          type_id:  product.type_id,
          name:     product.name,
          quantity: product.count,
          stored:   0,
          bonus:    0,
        });
      } else {
        result.push({
          type_id:  product.type_id,
          name:     product.name,
          quantity: product.count,
          stored:   material.quantity,
          bonus:    material.material,
        });
      }
    }

    return result;
  }

  public storage(type_id: number): IStorageEntry {
    let stored = this.stored.find(x => x.type_id === type_id);
    return <IStorageEntry>stored || <IStorageEntry>{  };
  }

  public required_materials(groups: ItemGroup[]): IDependency[] {
    if (groups.length === 0 || groups[0] === ItemGroup.All) {
      groups = this.all_groups();
    }

    let stored = this.stored
      .filter(x => groups.indexOf(x.group_id || 0) >= 0);

    return this.raw
      .filter(x => groups.indexOf(x.group_id || 0) >= 0)
      .map(x => {
        let s = stored.find(y => y.type_id === x.ptype_id);
        x.stored = s ? s.quantity : 0;
        return x;
      });
  }

  public all_groups(): ItemGroup[] {
    return [
      ItemGroup.Minerals,
      ItemGroup.Ice,
      ItemGroup.Moon,
      ItemGroup.Gas,
      ItemGroup.Salvage,
      ItemGroup.PI0Solid,
      ItemGroup.PI0Liquid,
      ItemGroup.PI0Organic,
      ItemGroup.PI1,
      ItemGroup.PI2,
      ItemGroup.PI3,
      ItemGroup.PI4,
    ];
  }

  public async kick_member(cid: number) {
    await axios.delete(`${BASE_PATH}/${this.pid}/members/${cid}`)
    this.members = (await axios.get(`${BASE_PATH}/${this.pid}/members`)).data;
  }

  public async import_blueprints() {
    this.busy = true;

    await axios.put(`${BASE_PATH}/${this.pid}/blueprints/import`);
    //await this.god();

    this.busy = false;
  }

  public static async budget_entry(
    pid: string,
    bid: string
  ): Promise<IBudgetEntry> {
    return (await axios.get(`${BASE_PATH}/${pid}/budget/${bid}`)).data;
  }

  public async budget_remove_entry(tid: BudgetId): Promise<void> {
    this.busy = true;
    await axios.delete(`${BASE_PATH}/${this.pid}/budget/${tid}`);
    //await this.god();
    this.busy = false;
  }

  // Loads all blueprints that are required for the project
  public async load_blueprints(): Promise<IBlueprint[]> {
    this.busy = true;
    const res = (await axios.get(`${BASE_PATH}/${this.pid}/blueprints`)).data;
    this.busy = false;
    return res;
  }

  // Loads all stored blueprints
  public async load_stored_blueprints(): Promise<IBlueprintEntry[]> {
    this.busy = true;
    const bps = (await axios.get(`${BASE_PATH}/${this.pid}/blueprints/stored`)).data;

    for (let bp of bps) {
      let saved = this.blueprints.find(y => y.type_id === bp.btype_id);

      if (!saved) {
        continue;
      } else {
        saved.me     = bp.me;
        saved.te     = bp.te;
        saved.runs   = bp.runs;
        saved.stored = true;
      }
    }

    this.busy = false;
    return this.blueprints;
  }

  public async add_budget_entry(data: IAddBudgetEntry): Promise<void> {
    this.busy = true;
    await axios.post(`${BASE_PATH}/${this.pid}/budget`, data);
    this.busy = false;

    await this.load_budget_entries();
  }

  public async load_budget_entries(): Promise<IBudgetEntry[]> {
    this.busy = true;
    const res = (await axios.get(`${BASE_PATH}/${this.pid}/budget`)).data;
    this.busy = false;

    this.budget_entries = res;
    return res;
  }

  private async load(): Promise<void> {
    this.busy = true;

    await axios.get(`${PROJECT_PATH}/${this.pid}`)
      .then(x => this.info = x.data)
      /*.then((x: any) => {
        let a = x.data;
        this.info = a;
        //this.stored = a.materials_stored;
        //this.raw = a.materials_raw;
        //this.buildsteps = a.buildsteps;
        //this.members = a.members;
        //this.market = a.market;
        //this.blueprints = a.bp_required;
      })*/;
    //this.stored = await Service.stored(this.pid);

    //await this.load_stored_blueprints();
    //await this.load_budget_entries();

    this.load_promise = null;
    this.busy = false;

    return;
  }
}

export type ProjectId   = Uuid;
export type BudgetCategory = 'PURCHASE' | 'SOLD' | 'MANUFACTURE' | 'RESEARCH' | 'OTHER';

export interface IAddBudgetEntry {
  // User that created this cost
  character:   CharacterId,
  // Cost amount
  amount:      number,
  // Category of the budget
  category:    BudgetCategory,

  // Short description for what the cost was
  description: String,
}

export interface IBudgetEntry {
  // Unique id of the tracking entry
  budget:      string;
  // Cost amount
  amount:      number;
  // User that created this cost
  character:   CharacterId;
  // Timestamp when this tracking was created
  created_at:  number;
  // Category of the budget entry
  category:    BudgetCategory;
  // Short description for what the cost was
  description: String;
}

export interface BlueprintStorageEntry {
  type_id:  TypeId;
  group_id: number;
  runs?:    number;
  me?:      number;
  te?:      number;
  name:     string;
}

export interface IBlueprintEntry {
  type_id:        TypeId;
  name:           string;
  stored:         boolean;
  is_reaction:    boolean;
  is_manufacture: boolean;

  runs?:          number;
  me?:            number;
  te?:            number;
}

export interface IAppraisal {
    sell_price:  number;
    split_price: number;
    buy_price:   number;

    items:       IAppraisalItem[];

    code?:       string;
    uri?:        string;
}

export interface IAppraisalItem {
    type_id:     number;
    name:        string;
    amount:      number;

    sell_price:  number;
    split_price: number;
    buy_price:   number;
}
