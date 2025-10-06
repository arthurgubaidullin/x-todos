import type { Todo } from "./todo.js";

export interface Todos {
  readonly list: ReadonlyArray<Todo>;
}
