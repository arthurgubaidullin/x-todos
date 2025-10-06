<script setup lang="ts">
import { FetchClient } from "@x-todos/fetch-client";
import NewTodoForm from "./NewTodoForm.vue";
import TodoList from "./TodoList.vue";
import type { WebClient } from "@x-todos/web-client";

const client: WebClient = new FetchClient("http://localhost:5173/api/todos");
</script>

<template>
  <div>
    <TodoList />
    <hr />
    <NewTodoForm :client="client" />
  </div>
</template>
