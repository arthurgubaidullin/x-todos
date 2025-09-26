<script setup lang="ts">
import { Todos } from "todos-wasm";
import { nanoid } from "nanoid";
import { onMounted, shallowRef, toValue, watchEffect } from "vue";

interface Todo {
  readonly text: () => string;
}

interface TodoService {
  readonly all: () => Array<Todo>;
  readonly add: (id: string, text: string) => void;
}

let todos: TodoService = new Todos();

let list = shallowRef<Array<Todo>>([]);

onMounted(() => {
  list.value = todos.all();
});

watchEffect(() => {
  console.log(toValue(list));
});

const add = () => {
  todos.add(nanoid(), "lol");
  list.value = todos.all();
};

defineProps<{ msg: string }>();
</script>

<template>
  <h1>{{ msg }}</h1>

  <div class="card">
    <button type="button" @click="add">Add!</button>
    <p>
      Edit
      <code>components/HelloWorld.vue</code> to test HMR
    </p>
  </div>

  <p>
    Check out
    <a href="https://vuejs.org/guide/quick-start.html#local" target="_blank"
      >create-vue</a
    >, the official Vue + Vite starter
  </p>
  <p>
    Learn more about IDE Support for Vue in the
    <a
      href="https://vuejs.org/guide/scaling-up/tooling.html#ide-support"
      target="_blank"
      >Vue Docs Scaling up Guide</a
    >.
  </p>
  <p class="read-the-docs">Click on the Vite and Vue logos to learn more</p>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
