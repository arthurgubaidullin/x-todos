<script setup lang="ts">
import type { Todo } from "@x-todos/vue-client-interface";

const props = defineProps<{ data: Todo }>();

const todo = props.data.text;
</script>

<template>
  <div>{{ todo }}</div>
</template>
