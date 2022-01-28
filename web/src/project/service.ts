import {CharacterId, GroupId, ItemGroup, ItemId, SystemId, BudgetId, TypeId, Uuid} from '@/utils';
import axios from 'axios';

// Base path that is used for all requests
const BASE_ADDR = '/api/v1/projects';

export class Project {
  private load_promise: Promise<void> | null = null;

  public info: IProject = <IProject>{  };
  public raw: IMaterial[] = [];
  public buildsteps: IBuildstep = <IBuildstep>{  };
  public stored: IMaterial[] = [];
  public members: IMember[] = [];

  public market: Map<String, IProjectMarket[]> = new Map();

  public blueprints: Blueprint[] = [];

  constructor(
    public pid: ProjectId
  ) {
    this.load_promise = this.by_id();
  }

  public async init(): Promise<void> {
    return this.load_promise
      ? this.load_promise
      : new Promise<void>((r, _) => r());
  }

  public required_blueprints(
    filter: 'ALL' | 'BLUEPRINTS' | 'REACTIONS'
  ): Blueprint[] {
    return this.blueprints
      .filter(x => {
        if (filter === 'BLUEPRINTS') {
          return x.is_manufacture();
        } else if (filter === 'REACTIONS') {
          return x.is_reaction();
        } else {
          return true;
        }
      })
      .sort((a, b) => a.name().localeCompare(b.name()));
  }

  public has_blueprint(
    filter: 'BLUEPRINTS' | 'REACTIONS'
  ): boolean {
    return this.blueprints.find(x => {
      if (filter === 'BLUEPRINTS') {
        return x.is_manufacture()
      } else if (filter === 'REACTIONS') {
        return x.is_reaction()
      } else {
        return true;
      }
    }) !== undefined;
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

  public required_materials(groups: ItemGroup[]): IRequiredMaterial[] {
    if (groups.length === 0 || groups[0] === ItemGroup.All) {
      groups = this.all_groups();
    }

    let stored = this.stored
      .filter(x => groups.indexOf(x.group_id || 0) >= 0);

    return this.raw
      .filter(x => groups.indexOf(x.group_id || 0) >= 0)
      .map(x => {
        let s = stored.find(y => y.type_id === x.type_id);
        let s_count = s ? s.quantity : 0;

        return {
          type_id:  x.type_id,
          name:     x.name,
          quantity: x.quantity,
          group_id: x.group_id,
          stored:   s_count,
          bonus:    x.material
        };
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

  public market_info(
    filter: 'BUY' | 'SELL',
    sid:    SystemId
  ): IProjectMarket[] {
    return this.market.get(filter + sid) || [];
  }

  public market_total_min(
    filter: 'BUY' | 'SELL',
    sid:    SystemId
  ): number {
    return (this.market.get(filter + sid) || [])
      .map(x => x.a_min)
      .reduce((acc, x) => acc += x, 0);
  }

  public market_total_avg(
    filter: 'BUY' | 'SELL',
    sid:    SystemId
  ): number {
    return (this.market.get(filter + sid) || [])
      .map(x => x.a_avg)
      .reduce((acc, x) => acc += x, 0);
  }

  public market_total_max(
    filter: 'BUY' | 'SELL',
    sid:    SystemId
  ): number {
    return (this.market.get(filter + sid) || [])
      .map(x => x.a_max)
      .reduce((acc, x) => acc += x, 0);
  }

  public async kick_member(cid: number) {
    await axios.delete(`${BASE_ADDR}/${this.pid}/members/${cid}`)
    this.members = (await axios.get(`${BASE_ADDR}/${this.pid}/members`)).data;
  }

  private async by_id(): Promise<void> {
    let sid = 30000142; // Jita

    // TODO: refactor
    await axios.get(`${BASE_ADDR}/${this.pid}`)
      .then(x => this.info = x.data)
      .then(_ => axios.get(`${BASE_ADDR}/${this.pid}/materials/stored`))
      .then(x => this.stored = x.data)
      .then(_ => axios.get(`${BASE_ADDR}/${this.pid}/materials/raw`))
      .then(x => this.raw = x.data)
      .then(_ => axios.get(`${BASE_ADDR}/${this.pid}/buildsteps`))
      .then(x => this.buildsteps = x.data)
      .then(_ => axios.get(`${BASE_ADDR}/${this.pid}/members`))
      .then(x => this.members = x.data)
      .then(_ => ProjectService2.market_buy(this.pid, sid))
      .then(x => this.market.set('BUY' + sid, x))
      .then(_ => ProjectService2.market_sell(this.pid, sid))
      .then(x => this.market.set('SELL' + sid, x))

    let blueprint_info = (await axios.get(`${BASE_ADDR}/${this.pid}/blueprints/info`)).data;
    this.blueprints = (await axios.get(`${BASE_ADDR}/${this.pid}/blueprints/required`))
      .data
      .map((x: IBlueprint) => {
        let info = blueprint_info
          .find((x: IBlueprintInfo) => x.type_id === x.type_id);
        return new Blueprint(x, info);
      });

    /*await Promise.all([
      info,
      stored,
      raw,
      buildsteps,
      members,
      buy,
      sell
    ])
    .then(_ => console.log('Loaded shit \'n stuff'));*/

    this.load_promise = null;
    return;
  }
}

export class Blueprint {
  constructor(
    public blueprint: IBlueprint,
    public info:      IBlueprintInfo | undefined
  ){  }

  public name(): string {
    return this.blueprint.name;
  }

  public type_id(): number {
    return this.blueprint.type_id;
  }

  public iters(): number {
    return this.blueprint.iters;
  }

  public is_manufacture(): boolean {
    return this.blueprint.is_manufacture;
  }

  public is_reaction(): boolean {
    return this.blueprint.is_reaction;
  }

  public typ(): string {
    if (this.info) {
      if (this.info.original === null || this.info.original) {
        return 'bp';
      } else {
        return 'bpc';
      }
    } else {
      return 'bp';
    }
  }

  public runs(): string | number {
    if (this.info && this.info.original) {
      return '∞';
    } else if (this.info && !this.info.original) {
      return this.info.runs;
    } else {
      return '';
    }
  }

  public material_eff(): string | number {
    if (this.info) {
      return this.info.material_eff;
    } else {
      return '';
    }
  }

  public time_eff(): string | number {
    if (this.info) {
      return this.info.time_eff;
    } else {
      return '';
    }
  }

  public is_stored(): boolean {
    return this.info ? true : false;
  }
}

export class ProjectService2 {
  public static cache: Map<string, Project> = new Map();

  // Gets a specific project by its id.
  // If the project is already in the cache, the cached version will be
  // returned.
  public static async by_id(pid: ProjectId): Promise<Project> {
    let cache = this.cache.get(pid);
    if (cache) {
      return cache;
    }

    let project = new Project(pid);
    this.cache.set(pid, project);
    return project;
  }

  // Gets all projects that the user has access to.
  public static async get_all(): Promise<IInfo[]> {
    return (await axios.get(`${BASE_ADDR}`)).data;
  }

  // Creates a new project and fetches it.
  public static async create(info: IConfig): Promise<ProjectId> {
    let pid = (await axios.post(`${BASE_ADDR}`, info)).data;
    await this.by_id(pid);
    return pid;
  }

  public static async edit(
    pid: ProjectId,
    info: IConfig
  ): Promise<void> {
    await axios.put(`${BASE_ADDR}/${pid}`, info);
    await this.refresh(pid);
    return;
  }

  // Removes a project and removes it from the cache.
  public static async remove(pid: ProjectId): Promise<void> {
    await axios.delete(`${BASE_ADDR}/${pid}`);
    this.cache.delete(pid);
    return;
  }

  // Deletes the currently cached project and fetches it again.
  public static async refresh(pid: ProjectId): Promise<void> {
    this.cache.delete(pid);
    await this.by_id(pid);

    return;
  }

  public static async market_buy(
    pid: ProjectId,
    sid: SystemId
  ): Promise<IProjectMarket[]> {
    return (await axios.get(`${BASE_ADDR}/${pid}/market/${sid}/buy`)).data;
  }

  public static async market_sell(
    pid: ProjectId,
    sid: SystemId
  ): Promise<IProjectMarket[]> {
    return (await axios.get(`${BASE_ADDR}/${pid}/market/${sid}/sell`)).data;
  }

  public static async budget_entries(pid: string): Promise<IBudgetEntry[]> {
    return (await axios.get(`${BASE_ADDR}/${pid}/budget`)).data;
  }

  public static async budget_entry(
    pid: string,
    bid: string
  ): Promise<IBudgetEntry> {
    return (await axios.get(`${BASE_ADDR}/${pid}/budget/${bid}`)).data;
  }

  public static async budget_add_entry(pid: string, data: IAddBudgetEntry): Promise<void> {
    return (await axios.post(`${BASE_ADDR}/${pid}/budget`, data));
  }

  public static async budget_edit_entry(
    pid: string,
    bid: string,
    data: IAddBudgetEntry
  ): Promise<void> {

    return (
      await axios.put(`${BASE_ADDR}/${pid}/budget/${bid}`, data)
    );
  }

  public static async budget_remove_entry(pid: string, tid: BudgetId): Promise<void> {
    return await axios.delete(`${BASE_ADDR}/${pid}/budget/${tid}`);
  }

   public static async add_member(pid: string): Promise<void> {
     return await axios.post(`${BASE_ADDR}/${pid}/members`);
   }
}

export type ProjectId   = Uuid;
export type TemplateId  = Uuid;

export interface IProject {
  project:             ProjectId;
  name:                string;
  owner:               CharacterId;
  products:            IProduct[];
  stored_materials:    IMaterial[];
  raw_materials:       IMaterial[];
  required_blueprints: IBlueprint[];
}

export interface IInfo {
  project: ProjectId;
  name:    string;
  pinned:  boolean;
  status:  string;
  owner:   number;
}

export interface IConfig {
  name:       string;
  products:   IProduct[];
}

export interface IProduct {
  name:    string;
  count:   number;
  type_id: TypeId;
}

export interface IMaterial {
  type_id:  TypeId;
  quantity: number;
  name:     string;
  material?: number;

  bonus?:   number;
  stored?:  number;

  group_id?:     GroupId;
}

export interface IBlueprint {
  type_id:        TypeId;
  name:           string;
  stored:         boolean;
  is_reaction:    boolean;
  is_manufacture: boolean;
  iters:          number;
}

export interface IBlueprintInfo {
  type_id:      TypeId;
  original:     boolean;
  count:        number;
  runs:         number;
  material_eff: number;
  time_eff:     number;
}

export interface IProjectMarket {
  a_min:     number;
  a_max:     number;
  a_avg:     number;
  s_min:     number;
  s_max:     number;
  s_avg:     number;
  count:     number;
  type_id:   TypeId;
  name:      string;
}

// TODO: Refactor
export interface IBudgetEntry {
  budget:      string;
  character:   number;
  amount:      number;
  category:    string;
  description: string;
  created_at:  number;
}

export interface IAddBudgetEntry {
  character:   number;
  amount:      number;
  description: string;
}

export interface IRequiredMaterial {
  type_id:  TypeId;
  name:     string;
  quantity: number;
  stored:   number;
}

export interface IBuildstep {
  manufacture: IBuildstepEntry[];
  inventions:  IBuildstepEntry[];
}

export interface IBuildstepEntry {
  name:         string;
  type_id:      TypeId;
  time_per_run: number;
  time_total:   number;
  runs:         number;
  produces:     number;
  materials:    IBuildstepMaterial[];

  probability?: number;

  // Added on the client side
  open?:        boolean;
}

export interface IBuildstepMaterial {
  name:     String;
  type_id:  TypeId;
  quantity: number;
}

export interface IMember {
  character_id:     number,
  character_name:   string,
  corporation_id:   number,
  corporation_name: string,
  alliance_id?:     number,
  alliance_name?:   string,
}
