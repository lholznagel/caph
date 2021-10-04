import axios from 'axios';

export class AssetService {
  private static station_name_req: { [key: number]: any } = {};
  private static station_names: { [key: number]: string } = {};

  public static async by_id(
    iid: number
  ): Promise<IAsset> {
      return (await axios.get(`/api/asset/${iid}`)).data;
  }

  public static async asset_name(
    iid: number
  ): Promise<string> {
      return (await axios.get(`/api/asset/${iid}/name`)).data;
  }

  public static async assets(
    params: { [key: string]: any }
  ): Promise<ICharacterAsset[]> {
    return (
      await axios.get('/api/asset', { params })
    ).data;
  }

  public static async all_blueprints(
    params: { [key: string]: any }
  ): Promise<IAccountBlueprint[]> {
    return (
      await axios.get('/api/asset/blueprints', { params })
    ).data;
  }

  public static async blueprint(
    tid: number,
    iid: number | null | undefined
  ): Promise<IAccountBlueprint[]> {
    let path = iid
      ? `/api/asset/blueprints/${tid}/${iid}`
      : `/api/asset/blueprints/${tid}`;

    return (await axios.get(path)).data;
  }

  public static async blueprint_material(
    tid: number,
  ): Promise<IBlueprintMaterial[]> {
    return (await axios.get(`/api/asset/blueprints/${tid}/material`)).data;
  }

  public static async blueprint_product(
    tid: number,
  ): Promise<IBlueprintMaterial[]> {
    return (await axios.get(`/api/asset/blueprints/${tid}/product`)).data;
  }

  public static async location_name(
    sid: number
  ): Promise<string> {
    if (this.station_names[sid]) {
      return this.station_names[sid];
    } else if (this.station_name_req[sid]) {
      return this.station_name_req[sid];
    } else {
      this.station_name_req[sid] = axios.get(`/api/asset/location/${sid}/name`);
      this.station_names[sid] = (await this.station_name_req[sid]).data;
      this.station_name_req[sid] = null;
      return this.station_names[sid];
    }
  }
}

export interface IAsset {
  type_id:       number;
  item_id:       number;
  owner:         number;
  location_id:   number;
  quantity:      number;
  count:         number;
  location_flag: string;
  name:          string;

  reference_id?: number;
}

export interface ICharacterAsset {
  type_id:             number;
  item_ids:            number[];
  owners:              number[];
  location_ids:        number[];
  quantity:            number;
  count:               number;
  location_flag:       string;
  name:                string;

  // added by us
  key?:                number;
}

export interface IAccountBlueprint {
  type_id:             number;
  item_ids:            number[];
  owners:              number[];
  material_efficiency: number;
  time_efficiency:     number;
  quantity:            number;
  runs:                number;
  count:               number;
  name:                string;
}

export interface IBlueprint {
  type_id:             number;
  item_id:             number;
  material_efficiency: number;
  time_efficiency:     number;
  quantity:            number;
  runs:                number;
  name:                string;
}

export interface IBlueprintMaterial {
  type_id:    number;
  activity:   number;
  quantity:   number;
  is_product: boolean;
  name:       string;
}
