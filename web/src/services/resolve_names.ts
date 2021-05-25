import axios from 'axios';

const KV_NAME = 'resolve_ids';

export class IdNameCache {
  public static async resolve(id: number): Promise<string> {
    const localItems: IIdName[] = this.load();

    // check if the id is stored
    const item = localItems.find(x => x.id === id);
    if (item) {
      return item.name;
    }

    // fetch the name
    const nameReq = (await axios.get(`/api/items/resolve/${id}`)).data || { name: 'Unknown' };
    const name = nameReq.name;
    localItems.push({ id, name });

    this.save(localItems);

    return name;
  }

  public static async resolve_bulk(ids: number[]): Promise<IIdName[]> {
    const localItems: IIdName[] = this.load();
    const names: IIdName[]      = [];
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
      const items = (await axios.post(`/api/items/resolve/bulk`, unknown)).data;
      for (const item of items) {
        names.push({ id: item.item_id, name: item.name });
        localItems.push({ id: item.item_id, name: item.name });
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
    const localItems: IIdName[] = this.load();

    const item = localItems.find(x => x.id === id);
    if (item) {
      return item.name;
    } else {
      return 'Unknown';
    }
  }

  private static load(): IIdName[] {
    const localItems = localStorage.getItem(KV_NAME) || '[]';
    return JSON.parse(localItems);
  }

  private static save(kv: IIdName[]) {
    localStorage.setItem(KV_NAME, JSON.stringify(kv));
  }
}

export interface IIdName {
  id: number;
  name: string;
}
