export const PROJECT_CHANGE = 'project_change';
export const BUDGET_CHANGE  = 'budget_change';

export class Events {
  private events: any = {};

  public $on(name: string, fn: Function) {
    this.events[name] = fn;
  }

  public $emit(name: string, payload: any) {
    if (!this.events[name]) {
      return;
    }

    this.events[name](payload);
  }
}
