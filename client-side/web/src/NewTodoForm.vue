<script setup lang="ts">
import type { AddNewTodo } from "@x-todos/vue-client-interface";
import { ref } from "vue";

const props = defineProps<{ client: AddNewTodo }>();

const { client } = props;

const text = ref("");

const onSubmit = (event: Event) => {
  event.preventDefault();

  client.add({ text: text.value });

  text.value = "";
};
</script>

<template>
  <form v-on:submit="onSubmit">
    <h1>New Todo</h1>

    <label>
      Text:
      <input v-model="text" type="text" name="text" required />
    </label>

    <input type="submit" />
  </form>
</template>
