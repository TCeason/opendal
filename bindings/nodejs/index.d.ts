/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export class OperatorFactory {
  static memory(): Operator
}
export class ObjectMeta {
  location: string
  lastModified: number
  size: number
}
export class Operator {
  meta(path: string): Promise<ObjectMeta>
  write(path: string, content: Array<number>): Promise<void>
  read(path: string): Promise<Array<number>>
  delete(path: string): Promise<void>
}
