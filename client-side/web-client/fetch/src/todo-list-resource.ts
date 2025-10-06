import type { TodoResource } from "./todo-resource.js";

export type TodoListResource = Readonly<{
  items: ReadonlyArray<TodoResource>;
}>;
