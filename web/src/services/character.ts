import axios from 'axios';

const BASE_ADDR: string = '/api/v1/character';

export class CharacterService {
  private static character_name_req: { [key: number]: Promise<ICharacter> } = {  };
  private static character_names: { [key: number]: ICharacter } = {  };

  // TODO: move to auth service
  public static async whoami(): Promise<ICharacter> {
    return (await axios.get(`/api/v1/auth/whoami`)).data;
  }

  // TODO: move to auth service
  public static add() {
    window.location.href = `/api/v1/auth/login/alt`;
  }

  public static async alts(): Promise<ICharacter[]> {
    return (await axios.get(`${BASE_ADDR}/alts`)).data;
  }

  public static async remove(cid: number): Promise<void> {
    return (await axios.delete(`${BASE_ADDR}/${cid}`)).data;
  }

  public static async info(cid: number): Promise<ICharacter> {
    // Ignore what typescript says
    if (<any>this.character_name_req[cid]) {
      return this.character_name_req[cid];
    }

    if (this.character_names[cid]) {
      return this.character_names[cid];
    } else {
      this.character_name_req[cid] = axios
        .get(`${BASE_ADDR}/${cid}/info`)
        .then(x => x.data)
        .then(x => {
          this.character_names[cid] = x;
          delete this.character_name_req[cid];
          return x;
        });
      return this.character_name_req[cid];
    }
  }

  public static async refresh(cid: number): Promise<void> {
    return await axios.get(`${BASE_ADDR}/${cid}/refresh`);
  }

  public static async blueprints_total(): Promise<any[]> {
    return (await axios.get(`${BASE_ADDR}/blueprints/total`)).data;
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
