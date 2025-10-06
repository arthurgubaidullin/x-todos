import type { NewTodo } from "./new-todo.js";
import type { Todos } from "./todo-list.js";

export interface WebClient {
  readonly todos: () => Promise<Todos>;
  readonly add: (newTodo: NewTodo) => Promise<Todos>;
}
