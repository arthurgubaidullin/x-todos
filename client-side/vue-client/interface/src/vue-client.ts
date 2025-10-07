import type { AddNewTodo } from "./add-new-todo.js";
import type { GetTodos } from "./get-todos.js";

export interface VueClient extends GetTodos, AddNewTodo {}
