<script setup lang="ts">
import type { GetTodos } from "@x-todos/vue-client-interface";
import Todo from "./Todo.vue";

const props = defineProps<{ client: GetTodos }>();

const { client } = props;

const todos = client.todos;
</script>

<template>
  <h1>Todo list</h1>
  <ul>
    <li v-if="todos !== null" v-for="todo in todos" :key="todo.id">
      <Todo :data="todo" />
    </li>
  </ul>
</template>
