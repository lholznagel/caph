import axios from 'axios';

export class ItemService {
  public static async assets(): Promise<IItemEntry[]> {
    return (await axios.get('/api/items')).data;
  }

  public static async keys(): Promise<number[]> {
    return (await axios.get('/api/items/keys')).data;
  }
}

export interface IItemEntry {
  category_id: number;
  group_id:    number;
  item_id:     number;
}
