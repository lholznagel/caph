export let formatNumber = (numberToFormat: string): string => {
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
