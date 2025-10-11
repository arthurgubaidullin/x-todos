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

  async update(this: this, data: Readonly<{ text: string }>): Promise<void> {
    await fetch(this.#data.links.update, {
      method: "POST",
      body: JSON.stringify({ text: data.text }),
      headers: {
        "Content-Type": "application/json",
        "If-Match": `"${this.#data.version}"`,
      },
    });
  }

  async remove(this: this): Promise<void> {
    await fetch(this.#data.links.remove, {
      method: "DELETE",
      headers: {
        "Content-Type": "application/json",
      },
    });
  }
}
