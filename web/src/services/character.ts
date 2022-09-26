import axios from 'axios';

const AUTH_BASE_PATH: string      = '/api/v1/auth';
const CHARACTER_BASE_PATH: string = '/api/v1/character';

export class CharacterService {
  private static character_req: { [key: number]: Promise<ICharacterInfo> } = {  };
  private static characters: { [key: number]: ICharacterInfo } = {  };
  private static corporation_req: { [key: number]: Promise<ICorporationInfo> } = {  };
  private static corporations: { [key: number]: ICorporationInfo } = {  };

  public static add() {
    window.location.href = `${AUTH_BASE_PATH}/login/alt`;
  }

  public static async whoami(): Promise<ICharacter> {
    return (await axios.get(`${AUTH_BASE_PATH}/whoami`)).data;
  }

  public static async alts(): Promise<ICharacter[]> {
    return (await axios.get(`${CHARACTER_BASE_PATH}/alts`)).data;
  }

  public static async remove(cid: number): Promise<void> {
    return (await axios.delete(`${CHARACTER_BASE_PATH}/${cid}`)).data;
  }

  public static async refresh(cid: number): Promise<void> {
    return await axios.get(`${CHARACTER_BASE_PATH}/${cid}/refresh`);
  }

  public static async available_scopes(): Promise<IScope[]> {
    return (await axios.get(`${AUTH_BASE_PATH}/scopes/available`)).data;
  }

  public static async info(cid: number): Promise<ICharacterInfo> {
    // Ignore what typescript says
    if (<any>this.character_req[cid]) {
      return this.character_req[cid];
    }

    if (this.characters[cid]) {
      return this.characters[cid];
    } else {
      this.character_req[cid] = axios
        .get(`${CHARACTER_BASE_PATH}/${cid}/info`)
        .then(x => x.data)
        .then(x => {
          this.characters[cid] = x;
          delete this.character_req[cid];
          return x;
        });
      return this.character_req[cid];
    }
 }

  public static async corporation_info(cid: number): Promise<ICorporationInfo> {
    // Ignore what typescript says
    if (<any>this.corporation_req[cid]) {
      return this.corporation_req[cid];
    }

    if (this.corporations[cid]) {
      return this.corporations[cid];
    } else {
      this.corporation_req[cid] = axios
        .get(`${CHARACTER_BASE_PATH}/corporation/${cid}/info`)
        .then(x => x.data)
        .then(x => {
          this.corporations[cid] = x;
          delete this.corporation_req[cid];
          return x;
        });
      return this.corporation_req[cid];
    }
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

export interface ICharacterInfo {
  alliance_id:    number;
  corporation_id: number;
  name:           string;
}

export interface ICorporationInfo {
  alliance_id:    number;
  name:           string;
}

export interface IScope {
  key:    string;
  name:   string;
  reason: string;
  scopes: string[];
}
