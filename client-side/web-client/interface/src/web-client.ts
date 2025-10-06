import type { NewTodo } from "./new-todo.js";
import type { Todos } from "./todo-list.js";
import type { Todo } from "./todo.js";

export interface WebClient {
  readonly todos: () => Promise<Todos>;
  readonly add: (newTodo: NewTodo) => Promise<Todo>;
}
