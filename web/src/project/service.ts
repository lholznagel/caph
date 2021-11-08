import {CharacterId, GroupId, ItemId, LocationId, SystemId, TrackingId, TypeId, Uuid} from '@/utils';
import axios from 'axios';

// Base path that is used for all requests
const BASE_ADDR = '/api/v1/projects';

export class Project {
  private load_promise: Promise<void> | null = null;

  public info: IProject             = <IProject>{  };
  public stored: IProjectMaterial[] = [];

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

  private async by_id(): Promise<void> {
    this.info = (await axios.get(`${BASE_ADDR}/${this.pid}`)).data;
    this.stored = (await axios.get(`${BASE_ADDR}/${this.pid}/materials/stored`)).data;

    this.load_promise = null;
    return;
  }
}

export class ProjectService {
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

    /*let project = (await axios.get(`${BASE_ADDR}/${pid}`)).data;
    project.raw_materials = await this.raw_materials(pid);
    project.stored_materials = await this.stored_materials(pid);
    project.required_blueprints = await this.required_blueprints(pid);
    this.cache.set(pid, project);

    return project;*/
  }

  // Gets all projects that the user has access to.
  public static async get_all(): Promise<IProjectInfo[]> {
    return (await axios.get(`${BASE_ADDR}`)).data;
  }

  // Creates a new project and fetches it.
  public static async create(info: IProjectConfig): Promise<ProjectId> {
    let pid = (await axios.post(`${BASE_ADDR}`, info)).data;
    await this.by_id(pid);
    return pid;
  }

  public static async edit(
    pid: ProjectId,
    info: IProjectConfig
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

  public static async buildsteps(
    pid: ProjectId,
    activity: string
  ): Promise<IProjectBuildstep[]> {
    return (await axios.get(`${BASE_ADDR}/${pid}/buildsteps/${activity}`)).data;
  }

  public static async trackings(pid: string): Promise<IProjectCostTracking[]> {
    return (await axios.get(`${BASE_ADDR}/${pid}/cost/trackings`)).data;
  }

  public static async tracking_add(pid: string, data: IProjectAddCostTracking): Promise<void> {
    return (await axios.post(`${BASE_ADDR}/${pid}/cost/trackings`, data));
  }

  public static async tracking_edit(pid: string, data: IProjectCostTracking): Promise<void> {
    return (
      await axios.put(`${BASE_ADDR}/${pid}/cost/trackings/${data.id}`, data)
    );
  }

  public static async tracking_remove(pid: string, tid: TrackingId): Promise<void> {
    return await axios.delete(`${BASE_ADDR}/${pid}/cost/trackings/${tid}`);
  }

  // Gets a list of all raw materials
  private static async raw_materials(pid: ProjectId): Promise<IProjectMaterial[]> {
    return (await axios.get(`${BASE_ADDR}/${pid}/materials/raw`)).data;
  }

  // Gets all blueprints that are required
  private static async required_blueprints(
    pid: ProjectId
  ): Promise<IProjectBlueprint[]> {
    return (await axios.get(`${BASE_ADDR}/${pid}/blueprints/required`)).data;
  }
}

export type ProjectId   = Uuid;
export type TemplateId  = Uuid;

export interface IProject {
  id:                  ProjectId;
  name:                string;
  owner:               CharacterId;
  containers:          ItemId[];
  products:            IProjectProduct[];
  stored_materials:    IProjectMaterial[];
  raw_materials:       IProjectMaterial[];
  required_blueprints: IProjectBlueprint[];
}

export interface IProjectInfo {
  id:     ProjectId;
  name:   string;
  pinned: boolean;
  status: string;
}

export interface IProjectConfig {
  name:       string;
  products:   IProjectProduct[];
  containers: ItemId[];
}

export interface IProjectProduct {
  name:    string;
  count:   number;
  type_id: TypeId;
}

export interface IProjectMaterial {
  type_id:       TypeId;
  quantity:      number;
  name:          string;
  material:      number;

  item_id?:      ItemId;
  container_id?: ItemId;
  location_id?:  LocationId;
  group_id?:     GroupId;
}

export interface IProjectBlueprint {
  type_id:       TypeId;
  name:          string;

  original?:     boolean;
  count?:        number;
  runs?:         number;
  material_eff?: number;
  time_eff?:     number;
  container_id?: ItemId;
  location_id?:  LocationId;
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

export interface IProjectBuildstep {
  type_id: TypeId;
  name:    string;
  runs:    number;
  time:    number;
}

// TODO: Refactor
export interface IProjectCostTracking {
  id:          string;
  character:   number;
  amount:      number;
  description: string;
  created_at:  number;
}

export interface IProjectAddCostTracking {
  character:   number;
  amount:      number;
  description: string;
}
