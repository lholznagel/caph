import axios from 'axios';

const KV_NAME = 'resolve_locations';

export class CharacterService {
  private static character_name_req: any = null;
  private static character_names: { [key: number]: string } = {};

  public static async info(): Promise<ICharacter> {
    return (await axios.get('/api/auth/whoami')).data;
  }

  public static async alts(): Promise<ICharacter[]> {
    return (await axios.get('/api/character/alts')).data;
  }

  public static async ids(): Promise<number[]> {
    return (await axios.get('/api/character/ids')).data;
  }

  public static async remove(cid: number): Promise<void> {
    return (await axios.delete(`/api/character/${cid}`)).data;
  }

  public static async character_name(cid: number): Promise<string> {
    if (this.character_names[cid]) {
      return this.character_names[cid];
    } else if (this.character_name_req) {
      return this.character_name_req.then((x: any) => x.data);
    } else {
      this.character_name_req = axios.get(`/api/character/${cid}/name`);
      this.character_names[cid] = (await this.character_name_req).data;
      this.character_name_req = null;
      return this.character_names[cid];
    }
  }

  public static async refreshCharacter(cid: number): Promise<void> {
    return await axios.get(`/api/character/${cid}/refresh`);
  }
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
