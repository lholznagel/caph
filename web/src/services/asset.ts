import axios from 'axios';

export class AssetService {
  public static async assets(
    params: { [key: string]: any }
  ): Promise<IAsset[]> {
    console.log(params)
    return (await axios.get('/api/assets', { params })).data;
  }

  public static async blueprints(): Promise<IBlueprint[]> {
    return (await axios.get('/api/assets/blueprints')).data;
  }
}

export interface IAsset {
  type_id:             number;
  item_id:             number;
  location_id:         number;
  owners:              number[];
  quantity:            number;
  count:               number;
  name:                string;
}

export interface IBlueprint {
  type_id:             number;
  item_id:             number;
  location_id:         number;
  owners:              number[];
  material_efficiency: number;
  time_efficiency:     number;
  quantity:            number;
  runs:                number;
  count:               number;
  name:                string;
}

