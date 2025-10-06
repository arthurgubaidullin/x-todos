import type { NewTodo, Todos, WebClient } from "@x-todos/web-client";
import type { RootResource } from "./root-resource.js";
import type { TodoListResource } from "./todo-list-resource.js";

export class FetchClient implements WebClient {
  #baseUrl: string;

  constructor(baseUrl: string) {
    this.#baseUrl = baseUrl;
  }

  private async relations(this: this): Promise<RootResource> {
    const response = await fetch(this.#baseUrl);
    const data: RootResource = await response.json();

    return data;
  }

  async todos(this: this): Promise<Todos> {
    const relations = await this.relations();

    const response = await fetch(relations.list);
    const data: TodoListResource = await response.json();

    return data;
  }

  async add(this: this, newTodo: NewTodo): Promise<Todos> {
    const relations = await this.relations();

    await fetch(relations.create.replace(":id", crypto.randomUUID()), {
      method: "PUT",
      body: JSON.stringify(newTodo),
    });

    return await this.todos();
  }
}
