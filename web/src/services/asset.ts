import axios from 'axios';

export class AssetService {
  public static name_cache: { [key: number]: string } = {};

  public static async by_id(
    iid: number
  ): Promise<IAsset> {
      return (await axios.get(`/api/asset/${iid}`)).data;
  }

  public static async assets(
    params: { [key: string]: any }
  ): Promise<IGenericAsset[]> {
    return (
      await axios.get('/api/asset', { params })
    ).data;
  }

  public static async asset_name(
    iid: number
  ): Promise<string> {
    return (await axios.get(`/api/asset/${iid}/name`)).data;
  }

  public static async blueprint_material(
    tid: number
  ): Promise<IAssetBlueprintMaterial> {
      return (await axios.get(`/api/asset/${tid}/blueprint/material`)).data;
  }

  public static async blueprint_tree(
    tid: number
  ): Promise<any> {
      return (await axios.get(`/api/asset/${tid}/blueprint/tree`)).data;
  }

  public static async blueprint_flat(
    tid: number
  ): Promise<IAssetBlueprintFlat[]> {
    return (await axios.get(`/api/asset/${tid}/blueprint/flat`)).data;
  }

  public static async buildable_items(): Promise<IAssetBuildable[]> {
    return (
      await axios.get('/api/asset/all/buildable')
    ).data;
  }

  public static async resolve_id_from_name_bulk(names: string[], params: { [key: string]: any }): Promise<number[]> {
    return (await axios.post('/api/asset/resolve/id', names, { params })).data;
  }
}

export interface IAsset {
  type_id:       number;
  item_id:       number;
  owner:         number;
  location_id:   number;
  quantity:      number;
  volume:        number
  count:         number;
  category_id:   number;
  group_id:      number;
  location_flag: string;
  name:          string;

  reference_id?: number;

  original:      boolean;
  material_eff:  number;
  time_eff:      number;
  runs:          number;
}

export interface IGenericAsset {
  type_id:             number;
  item_ids:            number[];
  owners:              number[];
  location_ids:        number[];
  quantity:            number;
  count:               number;
  category_id:         number;
  group_id:            number;
  location_flag:       string;
  name:                string;

  original:            boolean;
  material_eff:        number;
  time_eff:            number;
  runs:                number;

  // added by us
  key?:                number;
  open?:               boolean;
}

export interface IAssetBuildable {
  type_id: number;
  name:    string;
}

export interface IAssetBlueprintRaw {
  type_id:  number;
  group_id: number;
  quantity: number;
  name:     string;
  stored:   number;
}

export interface IAssetBlueprintFlat {
  type_id:  number;
  mtype_id: number;
  name:     string;
}

export interface IAssetBlueprintMaterial {
  type_id:    number;
  quantity:   number;
  is_product: boolean;
}
