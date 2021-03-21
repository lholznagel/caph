import axios from 'axios';

const KV_NAME = 'resolve_ids';

export class IdNameCache {
  public static async resolve(itemId: number): Promise<IItem> {
    const localItems: IItem[] = this.load();

    // check if the id is stored
    const itemStored = localItems.find(x => x.itemId === itemId);
    if (itemStored) {
      return itemStored;
    }

    // fetch the id
    const item = (await axios.get(`/api/items/${itemId}`)).data;
    localItems.push(item);

    this.save(localItems);

    return item;
  }

  public static async resolve_bulk(itemIds: number[]): Promise<IItem[]> {
    const localItems: IItem[] = this.load();
    const items: IItem[]      = [];
    const unknown: number[]   = [];

    // check if the id is stored
    for (const id of itemIds) {
      const item = localItems.find(x => x.itemId === id);
      if (item) {
        items.push(item);
      } else {
        unknown.push(id);
      }
    }

    if (unknown.length > 0) {
      // fetch all unknown ids
      const itemsBulk = (await axios.post(`/api/items/bulk`, unknown)).data;
      for (const item of itemsBulk) {
        items.push({ categoryId: item.category_id, groupId: item.group_id, itemId: item.item_id });
        localItems.push(item);
      }

      this.save(localItems);
    }

    return items;
  }

  private static load(): IItem[] {
    const localItems = localStorage.getItem(KV_NAME) || '[]';
    return JSON.parse(localItems);
  }

  private static save(kv: IItem[]) {
    localStorage.setItem(KV_NAME, JSON.stringify(kv));
  }
}

export interface IItem {
  categoryId: number;
  groupId: number;
  itemId: number;
}
