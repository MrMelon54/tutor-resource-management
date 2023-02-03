const reISO = /^(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2}):(\d{2}(?:\.\d*))(?:Z|(\+|-)([\d|:]*))?$/;
const reMsAjax = /^\/Date\((d|-|.*)\)[\/|\\]$/;

export function dateTimeReviver(_: string, value: any) {
  var a;
  if (typeof value === "string") {
    a = reISO.exec(value);
    if (a != undefined) return new Date(value);
    a = reMsAjax.exec(value);
    if (a != undefined) {
      var b = a[1].split(/[-+,.]/);
      return new Date(b[0] ? +b[0] : 0 - +b[1]);
    }
  }
  return value;
}
