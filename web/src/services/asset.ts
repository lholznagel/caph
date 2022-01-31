import axios from 'axios';

export class AssetService {
  public static async buildable_items(): Promise<IAssetBuildable[]> {
    return (
      await axios.get('/api/asset/all/buildable')
    ).data;
  }
}

export interface IAssetBuildable {
  type_id: number;
  name:    string;
}
