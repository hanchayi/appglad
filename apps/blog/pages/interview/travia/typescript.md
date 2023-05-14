## Partial

将所有key改为可选

``` typescript
interface Test {
  a: string;
  b: string;
}

type Partial<T> = { P in keyof T ?: P[T] | undefined; }
```

## Pick

``` typescript
interface Todo {
  title: string;
  description: string;
  completed: boolean;
}

type TodoPreview = Pick<Todo, "title" | "completed">;

const todo: TodoPreview = {
  title: "Clean room",
  completed: false,
};

type Pick<V, K extends keyof V> = {[P in K]: V[P];}
```

## Omit

``` typescript
interface Todo {
  title: string;
  description: string;
  completed: boolean;
  createdAt: number;
}

type TodoPreview = Omit<Todo, "description">;

const todo: TodoPreview = {
  title: "Clean room",
  completed: false,
  createdAt: 1615544252770,
};

type Omit<V, K keyof V> = { [P in Exclude<keyof V, K>]: V[P]; }

```

## Required

``` typescript
interface Props {
  a: number;
  b: number;
}

type Required<T> = { [P in keyof T]-?: T[P];}
```

## Readonly

``` typescript
interface Todo {
  title: string;
}

type Readonly<T> = {readonly [P in keyof T]: P[T];}
```

## Record

``` typescript
interface CatInfo {
  age: number;
  breed: string;
}

type CatName = "miffy" | "boris" | "mordred";

const cats: Record<CatName, CatInfo> = {
  miffy: { age: 10, breed: "Persian" },
  boris: { age: 5, breed: "Maine Coon" },
  mordred: { age: 16, breed: "British Shorthair" },
};

type Record<K extends string | number | symbol, V> = { [P in Key]: V;};
```



