<script setup lang="ts">
import { FetchClient } from "@x-todos/fetch-client";
import { VueClient } from "@x-todos/vue-client";
import NewTodoForm from "./NewTodoForm.vue";
import TodoList from "./TodoList.vue";

const webClient = new FetchClient("/api/todos");
const client = new VueClient(webClient);
</script>

<template>
  <div>
    <TodoList :client="client" />
    <hr />
    <NewTodoForm :client="client" />
  </div>
</template>
