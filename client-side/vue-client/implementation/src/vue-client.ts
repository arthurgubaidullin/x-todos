import type {
  GetTodos,
  AddNewTodo,
  Todo,
  NewTodo,
} from "@x-todos/vue-client-interface";
import type { WebClient } from "@x-todos/web-client";
import { type Ref, ref, onMounted } from "vue";

export class VueClient implements GetTodos, AddNewTodo {
  #client: WebClient;
  #ref: Ref<ReadonlyArray<Todo>>;

  constructor(client: WebClient) {
    this.#client = client;
    this.#ref = ref([]);
  }

  get todos(): Readonly<Ref<ReadonlyArray<Todo>>> {
    onMounted(async () => {
      const todos = await this.#client.todos();

      this.#ref.value = todos.items;
    });
    return this.#ref;
  }

  add(newTodo: NewTodo): void {
    this.#client.add(newTodo).then((todo) => {
      this.#ref.value = [...this.#ref.value, todo];
    });
  }
}
