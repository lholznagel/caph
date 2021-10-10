import axios from 'axios';

export class ProjectService {
  public static async projects(): Promise<IProject[]> {
    return (await axios.get('/api/project')).data;
  }

  public static async create(project: IProject): Promise<string> {
    return (await axios.post('/api/project', project)).data;
  }

  public static async update(pid: string, project: IProject): Promise<void> {
    return (await axios.put(`/api/project/${pid}`, project));
  }

  public static async project(id: string): Promise<IProject> {
    return (await axios.get(`/api/project/${id}`)).data;
  }

  public static async containers(id: string): Promise<IProjectContainer[]> {
    return (await axios.get(`/api/project/${id}/containers`)).data;
  }

  public static async products(pid: string): Promise<IProjectProduct[]> {
    return (await axios.get(`/api/project/${pid}/products`)).data;
  }

  public static async remove(id: string): Promise<string> {
    return (await axios.delete(`/api/project/${id}`));
  }

  public static async required_blueprints(id: string): Promise<IProjectRequiredBlueprint[]> {
    return (await axios.get(`/api/project/${id}/blueprints/required`)).data;
  }

  public static async required_raw_materials(id: string): Promise<IProjectStoredMaterial[]> {
    return (await axios.get(`/api/project/${id}/raw`)).data;
  }

  public static async stored_materials(id: string): Promise<IProjectStoredMaterial[]> {
    return (await axios.get(`/api/project/${id}/materials/stored`)).data;
  }
}

export interface IProject {
  id?:        string;
  name:       string;
  containers: IProjectContainer[];
  products:   IProjectProduct[];
}

export interface IProjectContainer {
  item_id: number | null;
}

export interface IProjectProduct {
  type_id: number | null;
  count:   number;
}

export interface IProjectRequiredBlueprint {
  type_id:      number;
  name:         string;

  // Fields added by us
  status:       string;
  container_id: number;
  location_id:  number;
}

export interface IProjectStoredMaterial {
  container_id: number;
  group_id:     number;
  type_id:      number;
  quantity:     number;
  stored:       number;

  location_id?: number;
}
