<template>
  <div class="action-button-cell-renderer">
    <button class="btn" @click="editMovie">Edit</button>
    <button class="btn red-btn" @click="deleteMovie">Delete</button>
  </div>
</template>
  
<script setup>
import router from "@/router";

const props = defineProps({
  params: Object,
});

const editMovie = () => {
  router.push({
    name: "movies-edit",
    params: { id: props.params.data.id },
  });
};

const deleteMovie = () => {
  router.push({
    name: "movies-delete",
    params: { id: props.params.data.id },
  });
};
</script>
  
<style>
.action-button-cell-renderer .btn {
  padding: 0.5em 1em;
  background: #007bff;
  color: #fff;
  border: none;
  cursor: pointer;
  border-radius: 5px;
  width: 5rem;
}

.action-button-cell-renderer .btn:hover {
  background: #0056b3;
}

.red-btn {
  margin-left: 1rem;
  background: #f81111 !important;
}

.red-btn:hover {
  margin-left: 1rem;
  background: #871010 !important;
}
</style>