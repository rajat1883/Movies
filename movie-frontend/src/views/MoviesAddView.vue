<template>
  <div class="container">
    <h1>Edit Movie</h1>
    <form @submit.prevent="updateMovie" class="form">
      <div class="form-field">
        <label for="title">Title:</label>
        <input id="title" v-model="movie.title" />
      </div>

      <div class="form-field">
        <label for="releaseDate">Release Date:</label>
        <input id="releaseDate" v-model="movie.releaseDate" />
      </div>

      <div class="form-field">
        <label for="description">Description:</label>
        <input id="description" v-model="movie.description" />
      </div>

      <div class="form-field">
        <label for="genre">Genre:</label>
        <input id="genre" v-model="movie.genre" />
      </div>

      <div class="form-field">
        <label for="icon">Icon:</label>
        <input id="icon" v-model="movie.icon" />
      </div>

      <button type="submit" class="btn">Add</button>
    </form>
  </div>
</template>
  
<script setup>
import router from "@/router";
import { ref } from "vue";

const movie = ref({});

const updateMovie = async () => {
  movie.value.id = 0;
  const response = await fetch(`http://localhost:3000/movies`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(movie.value),
  });

  if (!response.ok) throw new Error("Failed to update movie");

  router.push({ title: "movies" });
};
</script>
  
<style scoped>
.container {
  width: 100%;
  max-width: 600px;
  margin: 0 auto;
  padding: 1em;
}

.form {
  display: flex;
  flex-direction: column;
  margin-top: 2em;
}

.form-field {
  margin-bottom: 1em;
}

.form-field label {
  display: block;
  margin-bottom: 0.5em;
}

.form-field input {
  width: 100%;
  padding: 0.5em;
  border: 1px solid #ccc;
  box-sizing: border-box;
}

.btn {
  width: 100%;
  padding: 0.5em 1em;
  background: #41dc8e;
  color: #fff;
  border: none;
  cursor: pointer;
  box-sizing: border-box;
}

.btn:hover {
  background: #198450;
}
</style>
  