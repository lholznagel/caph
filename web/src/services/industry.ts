import axios from 'axios';
import {NameService} from './name';

export class IndustryService {
  public static async jobs(): Promise<IIndustryJob[]> {
    return (await axios.get('/api/industry/jobs')).data;
  }

  public static async stations(): Promise<IStation[]> {
    const stations = (await axios.get('/api/industry/stations')).data;

    let names = [];
    for (const station of stations) {
      const name = await NameService.resolve(station);
      names.push(name);
    }

    let result = [];
    for(let i = 0; i < stations.length; i++) {
      const station: IStation = {
        label: names[i],
        value: stations[i]
      };
      result.push(station);
    }
    return result;
  }
}

export interface IStation {
  value: number;
  label: string;
}

export interface IIndustryJob {
  activity_id:       number;
  blueprint_type_id: number;
  start_date:        string;
  end_date:          string;
  installer_id:      number;
  facility_id:       number;
  job_id:            number;
  // Added by us
  remaining?:        number;
}
