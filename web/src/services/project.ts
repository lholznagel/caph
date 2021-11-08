import axios from 'axios';

export class ProjectService {
  public static async project(id: string): Promise<IProject> {
    return (await axios.get(`/api/project/${id}`)).data;
  }

  public static async projects(): Promise<IProject[]> {
    return (await axios.get('/api/project')).data;
  }

  public static async create_project(project: IProject): Promise<string> {
    return (await axios.post('/api/project', project)).data;
  }

  public static async project_cost(pid: string): Promise<IProjectCost> {
    return (await axios.get(`/api/project/${pid}/cost`)).data;
  }

  public static async project_stored_materials(pid: string): Promise<IProjectStored[]> {
    return (await axios.get(`/api/project/${pid}/stored`)).data;
  }

  public static async template(id: string): Promise<ITemplate> {
    return (await axios.get(`/api/project/templates/${id}`)).data;
  }

  public static async templates(): Promise<ITemplate[]> {
    return (await axios.get('/api/project/templates')).data;
  }

  public static async create_template(template: ITemplate): Promise<string> {
    return (await axios.post('/api/project/templates', template)).data;
  }

  public static async update_template(tid: string, template: ITemplate): Promise<void> {
    return (await axios.put(`/api/project/templates/${tid}`, template));
  }

  public static async delete_template(tid: string): Promise<void> {
    return (await axios.delete(`/api/project/templates/${tid}`));
  }

  public static async template_products(tid: string): Promise<ITemplateProduct[]> {
    return (await axios.get(`/api/project/templates/${tid}/products`)).data;
  }

  public static async template_required_blueprints(tid: string): Promise<ITemplateBlueprint[]> {
    return (await axios.get(`/api/project/templates/${tid}/blueprints`)).data;
  }

  public static async template_required_materials(tid: string): Promise<ITemplateMaterial[]> {
    return (await axios.get(`/api/project/templates/${tid}/materials`)).data;
  }

  public static async trackings(pid: string): Promise<IProjectCostTracking[]> {
    return (await axios.get(`/api/project/${pid}/cost/tracking`)).data;
  }

  public static async tracking_add(pid: string, data: IProjectCostTracking): Promise<void> {
    return (await axios.post(`/api/project/${pid}/cost/tracking`, data));
  }

  public static async tracking_delete(pid: string, tid: string): Promise<void> {
    return (await axios.delete(`/api/project/${pid}/cost/tracking/${tid}`));
  }
}

export interface IProject {
  id?:        string;
  name:       string;
  runs:       number;
  template:   string;
  containers: number[];

  products?:  ITemplateProduct[];
}

export interface IProjectCost {
  products:   IProjectSubCost[];
  materials:  IProjectSubCost[];
  total_cost: number;
  sell_price: number;
}

export interface IProjectSubCost {
  name:     string;
  type_id:  number;
  quantity: number;
  price:    number;
}

export interface IProjectStored {
  item_id: number;
  container_id: number;
  location_id: number;
  type_id: number;
  quantity: number;
}

export interface ITemplate {
  id?:        string;
  name:       string;
  products:   ITemplateProduct[];
}

export interface ITemplateContainer {
  item_id: number | null;
}

export interface ITemplateProduct {
  type_id: number | null;
  count:   number;
}

export interface ITemplateBlueprint {
  name: string;
  type_id: number;
}

export interface ITemplateMaterial {
  type_id:  number;
  group_id: number;
  quantity: number;
  namme:    string;

  stored?:  number;
}

export interface IProjectCostTracking {
  id?:         string;
  creator?:    number;
  amount:      number;
  description: string;
  created_at?: number;
}
