export interface Result<T> {
  result_code: number,
  message: string,
  data: T,
}
