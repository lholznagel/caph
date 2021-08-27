import axios from "axios";

export class AllianceService {
  public static async set_fitting(fitting: INewFittingGroup) {
    return (await axios.post(`/api/alliance/fittings`, fitting)).data;
  }

  public static async get_fittings(): Promise<IFittingGroup[]> {
    return (await axios.get(`/api/alliance/fittings`)).data;
  }

  public static async get_fitting(id: string): Promise<IFittingGroup> {
    return (await axios.get(`/api/alliance/fittings/${id}`)).data;
  }

  public static async delete_fitting(id: string) {
    await axios.delete(`/api/alliance/fittings/${id}`);
  }
}

export interface IFittingGroup {
  id?:         string;
  fittings:    IFitting[];
  name:        string;
  url:         string;

  how_to_fit?: string;
  how_to_fly?: string;
}

export interface IFitting {
  name:     string;
  type_ids: IFittingTypeId[];
}

export interface IFittingTypeId {
  can_use: boolean;
  type_id: number;
}

export interface INewFittingGroup {
  name:        string;
  url:         string;
  fittings:    { name: string; type_ids: number[]; }[];
  how_to_fly?: string;
  how_to_fit?: string;
}
