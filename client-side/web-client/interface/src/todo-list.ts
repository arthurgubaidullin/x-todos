import type { Todo } from "./todo.js";

export interface Todos {
  readonly items: ReadonlyArray<Todo>;
}
