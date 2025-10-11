import type { Todo as _Todo } from "@x-todos/vue-client-interface";
import type { Todo as TodoFromWebClient } from "@x-todos/web-client";

export interface TodoRemoved {
  readonly todoRemoved: (todoId: string) => void;
}

export interface TodoUpdated {
  readonly todoUpdated: (todo: _Todo) => void;
}

export class Todo implements _Todo {
  #data: TodoFromWebClient;
  #topic: TodoRemoved & TodoUpdated;

  constructor(topic: TodoRemoved & TodoUpdated, data: TodoFromWebClient) {
    this.#data = data;
    this.#topic = topic;
  }

  get id(): string {
    return this.#data.id;
  }

  get text(): string {
    return this.#data.text;
  }

  get version(): string {
    return this.#data.version;
  }

  async remove(this: this): Promise<void> {
    await this.#data.remove();

    this.#topic.todoRemoved(this.#data.id);
  }

  async update(this: this, data: { text: string }): Promise<void> {
    const todo = await this.#data.update(data);

    this.#topic.todoUpdated(new Todo(this.#topic, todo));
  }
}
