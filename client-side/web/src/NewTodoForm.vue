<script setup lang="ts">
const onSubmit = (event: Event) => {
  event.preventDefault();

  alert("Ok!");
};
</script>

<template>
  <form v-on:submit="onSubmit">
    <h1>New Todo</h1>

    <label>
      Text:
      <input type="text" name="text" required />
    </label>

    <input type="submit" />
  </form>
</template>
