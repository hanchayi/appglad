interface Todo {
  readonly title: string
  readonly description: string
  completed: boolean
}

type GetReadonlyKeys<T> = {
  [K in keyof T]:
    (<S>() => S extends { [Z in K]: T[Z] } ? 2 : 1) extends
    (<S>() => S extends { -readonly [Z in K]: T[Z] } ? 2: 1) ? never : K
}[keyof T];

type Keys = GetReadonlyKeys<Todo> // expected to be "title" | "description"
