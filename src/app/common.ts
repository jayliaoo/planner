export interface Result<T> {
  result_code: number,
  message: string,
  data: T,
}

export function format(date: Date) {
  let year = date.getFullYear()
  let month: number | string = date.getMonth()
  let day: number | string = date.getDate()
  if (month < 10) {
    month = '0' + month
  }
  if (day < 10) {
    day = '0' + day
  }
  return `${year}-${month}-${day}`
}
