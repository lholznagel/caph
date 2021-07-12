import axios from "axios";

export class CorporationService {
  public static async blueprints(): Promise<any[]> {
    const corpId = window.whoami.corp_id;
    return (await axios.get(`/api/corporation/${corpId}/blueprints`)).data;
  }

  public static async setBlueprints(blueprints: ICorporationBlueprint[]) {
    const corpId = window.whoami.corp_id;
    return await axios.post(`/api/corporation/${corpId}/blueprints`, blueprints);
  }

  public static async deleteBlueprints() {
    const corpId = window.whoami.corp_id;
    return await axios.delete(`/api/corporation/${corpId}/blueprints`);
  }
}

export interface ICorporationBlueprint {
  id?:                 string;
  location_id:         number;
  material_efficiency: number;
  quantity:            number;
  runs:                number;
  time_efficiency:     number;
  type_id:             number;
  corp_id:             number;
}
