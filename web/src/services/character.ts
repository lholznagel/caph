import axios from 'axios';
import {IdNameCache} from './resolve_names';

export class CharacterService {
  public static async assets(): Promise<IAsset[]> {
    const assets: IAsset[] = [];

    const serverAssets: IServerAsset[] = (await axios.get(`/api/character/assets`)).data;
    for (const asset of serverAssets) {
      const name = (await IdNameCache.resolve(asset.type_id));

      let icon = 'icon';
      if (name.indexOf('Blueprint') >= 0) {
        icon = 'bp';
      }

      assets.push({
        name,
        icon,
        ...asset
      });
    }

    return assets;
  }

  public static async asset_names(items: IAsset[]): Promise<IAssetName[]> {
    const names: IAssetName[] = [];

    const locationIds: number[] = [];
    for (const item of items) {
      for (const location of item.locations) {
        if (location.item_id > 1000000000000 && locationIds.indexOf(location.item_id) === -1) {
          locationIds.push(location.item_id);
        } else if (location.item_id < 1000000000000) {
          const name = (await IdNameCache.resolve(location.item_id));
          names.push({ item_id: location.item_id, name });
        }
      }
    }

    const assetNames = (
      await axios
        .post(`/api/character/assets/names`, locationIds)
      )
      .data;

    return names.concat(assetNames);
  }
}

interface IServerAsset {
  item_ids:      number[];
  type_id:       number;
  quantity:      number;
  locations:     ILocation[];
}

export interface IAssetName {
  item_id: number;
  name:    string;
}

export interface IAsset {
  quantity:      number;
  item_ids:      number[];
  type_id:       number;
  name:          string;
  icon:          string;
  locations:     ILocation[];
}

export interface ILocation {
  item_id:  number;
  quantity: number;
  typ:      string;
}
