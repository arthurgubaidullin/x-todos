<script setup lang="ts">
import type { Todo } from "@x-todos/vue-client-interface";

const props = defineProps<{ data: Todo }>();

const todo = props.data;
</script>

<template>
  <div>{{ todo.text }} — <button @click="todo.remove()">Remove</button></div>
</template>
