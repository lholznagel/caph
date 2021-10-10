import axios from 'axios';

export class IndustryService {
  public static async jobs(): Promise<IIndustryJob[]> {
    return (await axios.get('/api/industry/jobs')).data;
  }
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
