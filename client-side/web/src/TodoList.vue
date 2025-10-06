<script setup lang="ts">
import type { Todos, WebClient } from "@x-todos/web-client";
import Todo from "./Todo.vue";
import { onMounted, ref } from "vue";

const props = defineProps<{ client: WebClient }>();

const { client } = props;

const todos = ref<Todos | null>(null);

onMounted(async () => {
  const data = await client.todos();

  todos.value = data;
});
</script>

<template>
  <h1>Todo list</h1>
  <ul>
    <li v-if="todos !== null" v-for="todo in todos.items" :key="todo.id">
      <Todo :data="todo" />
    </li>
  </ul>
</template>
