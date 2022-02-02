import axios from 'axios';

export class ItemService {
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

  public static async resolve_id_from_name_bulk(names: string[], params: { [key: string]: any }): Promise<number[]> {
    return (await axios.post('/api/v1/items/resolve', names, { params })).data;
  }
}

export interface IItem {
  type_id: number;
  name:    string;
}
