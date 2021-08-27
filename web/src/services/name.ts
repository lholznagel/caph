import axios from 'axios';

const KV_NAME = 'resolve_ids';

export class NameService {
  public static async resolve(id: number): Promise<string> {
    const localItems: IName[] = this.load();

    // check if the id is stored
    const item = localItems.find(x => x.id === id);
    if (item) {
      return item.name;
    }

    // fetch the name
    const name = (await axios.get(`/api/name/resolve/${id}`)).data;
    localItems.push({ id, name });

    this.save(localItems);

    return name;
  }

  public static async resolve_bulk(ids: number[]): Promise<IName[]> {
    const localItems: IName[] = this.load();
    const names: IName[]      = [];
    const unknown: number[]     = [];

    // check if the id is stored
    for (const id of ids) {
      const item = localItems.find(x => x.id === id);
      if (item) {
        names.push({ id, name: item.name });
      } else {
        unknown.push(id);
      }
    }

    if (unknown.length > 0) {
      // fetch all unknown ids
      const items = (await axios.post(`/api/name/resolve/bulk`, unknown)).data;
      for (let i = 0; i < unknown.length; i++) {
        const obj = { id: unknown[i], name: items[i] };
        names.push(obj);
        localItems.push(obj);
      }

      this.save(localItems);
    }

    names
      .sort((a, b) => {
        if (a.name < b.name) {
          return -1;
        }
        if (a.name > b.name) {
          return 1;
        }
        return 0;
      });

    return names;
  }

  public static resolve_name_sync(id: number): string {
    const localItems: IName[] = this.load();

    const item = localItems.find(x => x.id === id);
    if (item) {
      return item.name;
    } else {
      return 'Unknown';
    }
  }

  public static async resolve_names_to_id(names: string[]): Promise<IName[]> {
    const localItems: IName[] = this.load();
    const result: IName[]     = [];
    const unknown: string[]   = [];

    // check if the id is stored
    for (const name of names) {
      const item = localItems.find(x => x.name === name);
      if (item) {
        result.push({ id: item.id, name });
      } else {
        unknown.push(name);
      }
    }

    if (unknown.length > 0) {
      // fetch all unknown ids
      const items = (await axios.post(`/api/name/resolve/bulk/id`, unknown)).data;
      for (let key of Object.keys(items)) {
        const obj = { id: Number(key), name: items[key] };
        result.push(obj);
        localItems.push(obj);
      }

      this.save(localItems);
    }

    result
      .sort((a, b) => {
        if (a.name < b.name) {
          return -1;
        }
        if (a.name > b.name) {
          return 1;
        }
        return 0;
      });

    return result;
  }

  private static load(): IName[] {
    const localItems = localStorage.getItem(KV_NAME) || '[]';
    return JSON.parse(localItems);
  }

  private static save(kv: IName[]) {
    localStorage.setItem(KV_NAME, JSON.stringify(kv));
  }
}

export interface IName {
  id: number;
  name: string;
}
