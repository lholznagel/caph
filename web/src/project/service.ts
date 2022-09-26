import { CharacterId, GroupId, SystemId, TypeId, Uuid } from '@/utils';
import axios from 'axios';
import { Project, ProjectId } from './project';

// Base path that is used for all requests
const BASE_ADDR: string    = '/api/v1/projects';
const PROJECT_PATH: string = '/api/v2/projects';

export const BUDGET_CATEGORIES: {
  label: string;
  value: string;
}[] = [{
  label: 'Purchase',
  value: 'PURCHASE'
}, {
  label: 'Sold',
  value: 'SOLD'
}, {
  label: 'Manufacture',
  value: 'MANUFACTURE'
}, {
  label: 'Research',
  value: 'RESEARCH'
}, {
  label: 'Other',
  value: 'OTHER'
}]

export class Service {
  public static cache: Map<string, Project> = new Map();

  // Gets all projects that the user has access to.
  public static async get_all(): Promise<IProjectInfo[]> {
    return (await axios.get(`${PROJECT_PATH}`)).data;
  }

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

  // Creates a new project and fetches it.
  public static async create(info: INewProject): Promise<ProjectId> {
    let pid = (await axios.post(`${PROJECT_PATH}`, info)).data;
    await this.by_id(pid);
    return pid;
  }

  public static async edit(
    pid: ProjectId,
    info: INewProject
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

  public static async budget_entry(
    pid: string,
    bid: string
  ): Promise<IBudgetEntry> {
    return (await axios.get(`${BASE_ADDR}/${pid}/budget/${bid}`)).data;
  }

  public static async budget_edit_entry(
    pid: string,
    bid: string,
    data: any
  ): Promise<void> {

    return (
      await axios.put(`${BASE_ADDR}/${pid}/budget/${bid}`, data)
    );
  }

  public static async add_member(pid: string): Promise<void> {
    return await axios.post(`${BASE_ADDR}/${pid}/members`);
  }

  public static async stored(pid: ProjectId): Promise<IStorageEntry[]> {
    return (await axios.get(`${BASE_ADDR}/${pid}/storage`)).data;
  }

  public static async storage_by_id(pid: ProjectId, tid: number): Promise<IStorageEntry> {
    return (await axios.get(`${BASE_ADDR}/${pid}/storage/${tid}`)).data;
  }

  public static async modify(pid: ProjectId, entries: IModify[], mode: string): Promise<IStorageEntry[]> {
    return await axios.put(`${BASE_ADDR}/${pid}/storage`, { mode, entries});
  }

  public static async set_storage(pid: ProjectId, modify: IModify[]): Promise<IStorageEntry[]> {
    return await axios.post(`${BASE_ADDR}/${pid}/storage`, modify);
  }

  public static async blueprints_import(pid: ProjectId): Promise<any> {
    return await axios.put(`${BASE_ADDR}/${pid}/blueprints/import`)
  }

  public static async aaa() {
    return await axios.get('/api/v2/industry/stockpile');
  }

  public static async jobs(pid: ProjectId): Promise<any> {
    return (await axios.get(`${PROJECT_PATH}/${pid}/jobs`)).data;
  }
}

export type TemplateId  = Uuid;

export interface IDependency {
  // Name of the product
  name:              string;
  // [TypeId] of the product
  ptype_id:          TypeId;
  // [GroupId] of the project
  group_id:          GroupId;
  // Number of products to produce
  products:          number;
  // Base requirements for building a single run
  products_base:     number;
  // Quantity that is produced with each iteration
  products_per_run:  number;
  // Time it takes to run one production cycle
  time_per_run:      number;

  // Materials that are required to build this component
  materials:         IDependency[];

  // Added by us, number of stored materials
  stored?:           number;
}

export interface IProject {
  project:             any;
  name:                string;
  owner:               CharacterId;
  products:            IProduct[];
  stored_materials:    IMaterial[];
  raw_materials:       IMaterial[];
  required_blueprints: IBlueprint[];
}

export interface IProjectInfo {
  project: ProjectId;
  name:    string;
  pinned:  boolean;
  status:  string;
  owner:   number;
}

export interface INewProject {
  name:       string;
  products:   IProduct[];
}

export interface IProduct {
  count:   number;
  meff:    number;
  name:    string;
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

export interface IModify {
  type_id:  TypeId;
  quantity: number;

  runs?:    number;
  me?:      number;
  te?:      number;
}

export interface IStorageEntry {
  type_id:  TypeId;
  group_id: GroupId;
  quantity: number;
  name:     string;

  runs?:    number;
  me?:      number;
  te?:      number;
}
