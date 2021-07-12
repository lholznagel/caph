export let formatNumber = (numberToFormat: number): string => {
    const splitted = numberToFormat.toString().split('.');
    const formatValue = splitted[0].split('').reverse().join('');
    const result: string[] = [];
    let count = 0;

    for (let i = 0; i < formatValue.length; i++) {
      if (count === 3) {
        result.push('.');
        count = 0;
      }
      result.push(formatValue.charAt(i));
      count += 1;
    }

    if (splitted[1]) {
      return result.reverse().join('') + ',' + splitted[1];
    } else {
      return result.reverse().join('');
    }
};

export let formatTime = (numberToFormat: number): string => {
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

