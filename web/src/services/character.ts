import axios from 'axios';

const KV_NAME = 'resolve_locations';

export class CharacterService {
  public static async info(): Promise<ICharacter> {
    return (await axios.get('/api/whoami')).data;
  }

  public static async alts(): Promise<ICharacter[]> {
    return (await axios.get('/api/character/alts')).data;
  }

  public static async ids(): Promise<number[]> {
    return (await axios.get('/api/character/ids')).data;
  }

  public static async char_name(id: number): Promise<string> {
    return (await axios.get(`/api/character/${id}/name`)).data;
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

export interface ICharacterAsset {
  type_id:             number;
  item_id:             number;
  location_id:         number;
  owners:              number[];
  quantity:            number;
  count:               number;
  name:                string;
}

export interface IItemLocation {
  // Added by us
  id?:       number;
  name:      string;
  owner_id:  number;
  system_id: number;
  type_id:   number;
}

export interface ICharacter {
  character:        string;
  character_id:     number;
  character_icon:   string;
  alliance:         string;
  alliance_id:      number;
  alliance_icon:    string;
  corporation:      string;
  corporation_icon: string;
  corporation_id:   number;
}

export const DEFAULT_CHARACTER: ICharacter = {
  character:        '',
  character_id:     0,
  character_icon:   '',
  alliance:         '',
  alliance_id:      0,
  alliance_icon:    '',
  corporation:      '',
  corporation_icon: '',
  corporation_id:   0,
};
