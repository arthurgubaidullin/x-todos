import type { Ref } from "vue";
import type { Todo } from "./todo.js";

export interface GetTodos {
  readonly todos: Readonly<Ref<ReadonlyArray<Todo>>>;
}
