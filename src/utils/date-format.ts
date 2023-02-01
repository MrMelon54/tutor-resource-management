export const monthNames = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
export const monthNamesShort = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
export const dayNames = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
export const dayNamesShort = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
const thString = ["th", "st", "nd", "rd"];

export function formatForLessonView(d: Date) {
  let a = thString[d.getDate()];
  if (!a) a = "th";
  return `${dayNamesShort[d.getDay()]} ${d.getDate()}${a} ${monthNamesShort[d.getMonth()]} ${d.getFullYear()} - ${timeStr(d)}`;
}
export function addDate(date: Date, days: number) {
  return new Date(date.getFullYear(), date.getMonth(), date.getDate() + days);
}

export function getDate(date: Date) {
  return addDate(date, 0);
}

export function timeSec(date: Date) {
  return date.getTime();
}

export function timeStr(date: Date) {
  let hrs = date.getHours();
  let min = date.getMinutes();
  return `${hrs}:${min < 10 ? "0" + min : min}${hrs < 12 ? "am" : "pm"}`;
}

export function dateTimeMerge(date: Date, time: Date) {
  return new Date(date.getFullYear(), date.getMonth(), date.getDate(), time.getHours(), time.getMinutes(), time.getSeconds(), time.getMilliseconds());
}
