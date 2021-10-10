import axios from 'axios';

export class UniverseService {
  public static async stations(): Promise<IStation[]> {
    return (await axios.get(`/api/universe/stations`)).data;
  }

  public static async station(
    sid: number
  ): Promise<IStation> {
    return (await axios.get(`/api/universe/stations/${sid}`)).data;
  }

  public static async add_station(
    station: IStation
  ): Promise<void> {
    return (await axios.post(`/api/universe/stations`, station)).data;
  }

  public static async delete_station(
    sid: number
  ): Promise<void> {
    return (await axios.delete(`/api/universe/stations/${sid}`)).data;
  }

  public static async systems(): Promise<ISystem[]> {
    return (await axios.get(`/api/universe/systems`)).data;
  }

  public static async system(sid: number): Promise<ISystem> {
    return (await axios.get(`/api/universe/systems/${sid}`)).data;
  }
}

export interface IStation {
  id:        number;
  name:      string;
  system_id: number;
  structure: string;

  pos?:      boolean;
}

export interface ISystem {
  id:   number;
  name: string;
}
