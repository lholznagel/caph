export type CharacterId = number;
export type GroupId     = number;
export type ItemId      = number;
export type LocationId  = number;
export type SystemId    = number;
export type TypeId      = number;

export type Uuid        = string;
export type BudgetId    = Uuid;

export enum ItemGroup {
  // Its hacky and I now it
  Blueprints = -2,
  NotCovered = -1,
  All        = 0,
  Minerals   = 18,
  Ice        = 423,
  Moon       = 427,
  Gas        = 711,
  Salvage    = 754,
  PI0Solid   = 1032,
  PI0Liquid  = 1033,
  PI0Organic = 1035,
  PI1        = 1042,
  PI2        = 1034,
  PI3        = 1040,
  PI4        = 1041,
}

export let format_number = (numberToFormat: number, with_comma: boolean = false): string => {
  const splitted = numberToFormat.toFixed(2).toString().split('.');
  const formatValue = splitted[0].split('').reverse().join('');
  const result: string[] = [];
  let count = 0;

  for (let i = 0; i < formatValue.length; i++) {
    let val = formatValue.charAt(i);

    if (val === '-' || val === '+') {
      result.push(val);
      continue
    }

    if (count === 3) {
      result.push('.');
      count = 0;
    }
    result.push(val);
    count += 1;
  }

  if (with_comma && splitted[1]) {
    return result.reverse().join('') + ',' + splitted[1];
  } else {
    return result.reverse().join('');
  }
};

export let format_time = (numberToFormat: number): string => {
  const DAY    = 60 * 60 * 24; // seconds * minutes * hours
  const HOUR   = 60 * 60;      // seconds * minutes
  const MINUTE = 60;           // seconds

  const days = Math.floor(numberToFormat / DAY);
  numberToFormat -= days * DAY;

  const hours = Math.floor(numberToFormat / HOUR);
  numberToFormat -= hours * HOUR;

  const minutes = Math.floor(numberToFormat / MINUTE);
  numberToFormat -= minutes * MINUTE;

  const seconds = numberToFormat;

  const preZero = (val: number): string => val >= 10 ? `${val}` : `0${val}`;

  if (days > 0) {
    return `${days}d ${preZero(hours)}h ${preZero(minutes)}m ${preZero(seconds)}s`;
  } else if (hours > 0) {
    return `${hours}h ${preZero(minutes)}m ${preZero(seconds)}s`;
  } else if (minutes > 0) {
    return `${minutes}m ${preZero(seconds)}s`;
  } else {
    return `${seconds}s`;
  }
};

export let format_date = (date_ms: number): string => {
  const date = new Date(date_ms);
  const preZero = (val: number): string => val >= 10 ? `${val}` : `0${val}`;
  const day = preZero(date.getUTCDate());
  const month = preZero(date.getMonth() + 1);
  const year = date.getFullYear();
  const hours = preZero(date.getUTCHours());
  const minutes = preZero(date.getUTCMinutes());

  return `${day}.${month}.${year} ${hours}:${minutes}`;
}
