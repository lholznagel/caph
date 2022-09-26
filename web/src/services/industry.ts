import axios from 'axios';

const INDUSTRY_JOB_PATH: string = '/api/v2/industry/jobs';

export class IndustryService {
  public static async character_jobs(): Promise<IIndustryJob[]> {
    return (await axios.get(`${INDUSTRY_JOB_PATH}`)).data;
  }

  public static async corporation_jobs(): Promise<IIndustryJob[]> {
    return (await axios.get(`${INDUSTRY_JOB_PATH}/corporation`)).data;
  }
}

export interface IIndustryJob {
  activity:              string;
  blueprint_type_id:     number;
  blueprint_location_id: number;
  status:                string;
  start_date:            string;
  end_date:              string;
  cost:                  number;
  runs:                  number;
  job_id:                number;
  corporation_id?:       number;
  remaining?:            number;
}
