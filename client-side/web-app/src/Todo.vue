<script setup lang="ts">
import type { Todo } from "@x-todos/vue-client-interface";
import { ref } from "vue";

const props = defineProps<{ data: Todo }>();

const todo = props.data;

const textToUpdate = ref(todo.text);

const submit = (event: Event) => {
  event.preventDefault();

  todo.update({ text: textToUpdate.value });
};
</script>

<template>
  <div>{{ todo.text }} — <button @click="todo.remove()">Remove</button></div>

  <div>
    <form v-on:submit="submit">
      <label>
        Text to update: <input name="text" v-model="textToUpdate" required />
      </label>
      <input type="submit" />
    </form>
  </div>
</template>
