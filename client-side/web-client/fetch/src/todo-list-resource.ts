import type { TodoResource } from "./todo-resource.js";

export type TodoListResource = Readonly<{
  list: ReadonlyArray<TodoResource>;
}>;
