import { TypeId } from '@/utils';
import axios from 'axios';

export class ItemService {
  private static cache: { [key: TypeId]: IItem } = {};

  public static async components(): Promise<IItem[]> {
    return (
      await axios.get('/api/v1/items/components')
    ).data;
  }

  public static async buildable_items(): Promise<IItem[]> {
    return (
      await axios.get('/api/v1/items/buildable')
    ).data;
  }

  public static async resolve_id(tid: TypeId): Promise<IItem> {
    if (this.cache[tid]) {
      console.log('cache hit');
      return this.cache[tid];
    } else {
      let entry: IItem = (
        await axios.post(`/api/v1/items/resolve/id/${tid}`, tid)
      ).data;
      this.cache[entry.type_id] = entry;
      return entry;
    }
  }

  public static async resolve_id_from_name_bulk(names: string[], params: { [key: string]: any }): Promise<number[]> {
    return (await axios.post('/api/v1/items/resolve', names, { params })).data;
  }
}

export interface IItem {
  type_id:     number;
  category_id: number;
  group_id:    number;
  volume:      number;
  name:        string;
}
