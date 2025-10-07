import type { Todo as _Todo } from "@x-todos/vue-client-interface";
import type { Todo as TodoFromWebClient } from "@x-todos/web-client";

export interface TodoRemoved {
  readonly todoRemoved: (todoId: string) => void;
}

export class Todo implements _Todo {
  #data: TodoFromWebClient;
  #topic: TodoRemoved;

  constructor(topic: TodoRemoved, data: TodoFromWebClient) {
    this.#data = data;
    this.#topic = topic;
  }

  get id(): string {
    return this.#data.id;
  }

  get text(): string {
    return this.#data.text;
  }

  async remove(this: this): Promise<void> {
    await this.#data.remove();

    this.#topic.todoRemoved(this.#data.id);
  }
}
