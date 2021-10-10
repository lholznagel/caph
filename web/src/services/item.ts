import axios from 'axios';

export class ItemService {
  public static async item_name(
    iid: number
  ): Promise<string> {
    return (await axios.get(`/api/item/${iid}/name`)).data;
  }
}
