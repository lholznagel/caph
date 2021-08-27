import axios from 'axios';

const KV_NAME = 'resolve_locations';

export class CharacterService {
  public static async assets(): Promise<any[]> {
    return (await axios.get('/api/character/assets')).data;
  }

  public static async blueprints(): Promise<ICharacterBlueprint[]> {
    return (await axios.get('/api/character/blueprints')).data;
  }

  public static async itemLocation(id: number): Promise<IItemLocation> {
    const localItems: IItemLocation[] = this.load();

    // check if the id is stored
    const item = localItems.find((x: IItemLocation) => x.id === id);
    if (item) {
      return item;
    }

    let location = (await axios.get(`/api/character/location/${id}`)).data;
    localItems.push({ id, ...location });
    this.save(localItems);
    return { id, ...location };
  }

  public static itemLocationByName(name: string): IItemLocation | undefined {
    const localItems: IItemLocation[] = this.load();
    return localItems.find((x: IItemLocation) => x.name === name);
  }

  private static load(): IItemLocation[] {
    const localItems = localStorage.getItem(KV_NAME) || '[]';
    return JSON.parse(localItems);
  }

  private static save(kv: IItemLocation[]) {
    localStorage.setItem(KV_NAME, JSON.stringify(kv));
  }
}

export interface ICharacterBlueprint {
  item_id:             number;
  location_flag:       string;
  location_id:         number;
  material_efficiency: number;
  quantity:            number;
  runs:                number;
  time_efficiency:     number;
  type_id:             number;
  user_id:             number;
}

export interface IItemLocation {
  // Added by us
  id?:       number;
  name:      string;
  owner_id:  number;
  system_id: number;
  type_id:   number;
}
