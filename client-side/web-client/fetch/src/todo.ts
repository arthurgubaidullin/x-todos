import type { Todo as _Todo } from "@x-todos/web-client";
import type { TodoResource } from "./todo-resource.js";

export class Todo implements _Todo {
  #data: TodoResource;

  constructor(data: TodoResource) {
    this.#data = data;
  }

  get id(): string {
    return this.#data.id;
  }

  get text(): string {
    return this.#data.text;
  }

  async remove(this: this): Promise<void> {
    await fetch(this.#data.remove, {
      method: "DELETE",
      headers: {
        "Content-Type": "application/json",
      },
    });
  }
}
