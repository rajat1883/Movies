<template>
  <div class="container">
    <h1>Edit Movie</h1>
    <form @submit.prevent="updateMovie" class="form">
      <div class="form-field">
        <label for="name">Name:</label>
        <input disabled id="name" v-model="movie.name" />
      </div>

      <div class="form-field">
        <label for="releaseDate">Release Date:</label>
        <input disabled id="releaseDate" v-model="movie.releaseDate" />
      </div>

      <div class="form-field">
        <label for="description">Description:</label>
        <input disabled id="description" v-model="movie.description" />
      </div>

      <div class="form-field">
        <label for="genre">Genre:</label>
        <input disabled id="genre" v-model="movie.genre" />
      </div>

      <div class="form-field">
        <label for="icon">Icon:</label>
        <input disabled id="icon" v-model="movie.icon" />
      </div>

      <button type="submit" class="btn">Delete</button>
    </form>
  </div>
</template>
  
<script setup>
import router from "@/router";
import { ref, onMounted } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();
const movie = ref({});

onMounted(async () => {
  const response = await fetch(
    `http://localhost:3000/movies/${route.params.id}`
  );
  if (!response.ok) throw new Error("Failed to load movie data");
  movie.value = await response.json();
});

const updateMovie = async () => {
  const response = await fetch(
    `http://localhost:3000/movies/${movie.value.id}`,
    {
      method: "DELETE",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(movie.value),
    }
  );

  if (!response.ok) throw new Error("Failed to update movie");

  router.push({ name: "movies" });
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
  background: #f81111;
  color: #fff;
  border: none;
  cursor: pointer;
  box-sizing: border-box;
}

.btn:hover {
  background: #871010;
}
</style>
  