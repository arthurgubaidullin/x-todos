<script setup lang="ts">
import { FetchClient } from "@x-todos/fetch-client";
import NewTodoForm from "./NewTodoForm.vue";
import TodoList from "./TodoList.vue";
import type { WebClient } from "@x-todos/web-client";
import { VueClient } from "@x-todos/vue-client";

const webClient: WebClient = new FetchClient("http://localhost:5173/api/todos");
const client = new VueClient(webClient);
</script>

<template>
  <div>
    <TodoList :client="client" />
    <hr />
    <NewTodoForm :client="client" />
  </div>
</template>
