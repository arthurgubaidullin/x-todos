import type { NewTodo } from "./new-todo.js";

export interface AddNewTodo {
  readonly add: (newTodo: NewTodo) => void;
}
