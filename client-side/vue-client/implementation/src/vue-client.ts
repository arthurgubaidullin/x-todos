import type {
  AddNewTodo,
  GetTodos,
  NewTodo,
  Todo as _Todo,
} from "@x-todos/vue-client-interface";
import type { WebClient } from "@x-todos/web-client";
import { type Ref, onMounted, shallowRef } from "vue";
import { type TodoRemoved, Todo } from "./todo.js";

export class VueClient implements GetTodos, AddNewTodo, TodoRemoved {
  #client: WebClient;
  #ref: Ref<ReadonlyArray<_Todo>>;

  constructor(client: WebClient) {
    this.#client = client;
    this.#ref = shallowRef([]);
  }

  get todos(): Readonly<Ref<ReadonlyArray<_Todo>>> {
    onMounted(async () => {
      const todos = await this.#client.todos();

      this.#ref.value = todos.items.map((todo) => new Todo(this, todo));
    });
    return this.#ref;
  }

  add(this: this, newTodo: NewTodo): void {
    this.#client.add(newTodo).then((todo) => {
      this.#ref.value = [...this.#ref.value, new Todo(this, todo)];
    });
  }

  todoRemoved(this: this, todoId: string): void {
    this.#ref.value = this.#ref.value.filter((todo) => todo.id !== todoId);
  }
}
