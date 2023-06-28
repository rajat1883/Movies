<template>
  <h1>MOVIES</h1>

  <input v-model="searchText" @input="onSearch" placeholder="Search..." />

  <br />
  <br />

  <ag-grid-vue
    class="ag-theme-alpine"
    :gridOptions="gridOptions"
    :rowData="movies"
    @grid-ready="onGridReady"
  />

  <br />

  <button class="btn green-btn" @click="addMovie">Add</button>
</template>

<script setup>
import { onMounted, ref } from "vue";
import { AgGridVue } from "ag-grid-vue3";
import ActionButtonsCellRenderer from "@/components/ActionButtonsCellRenderer.vue";
import router from "@/router";

const movies = ref([]);
const gridApi = ref(null);
const searchText = ref("");

const loadMovies = async () => {
  try {
    const response = await fetch("http://localhost:3000/movies");
    if (!response.ok) throw Error("No data available");
    movies.value = await response.json();
  } catch (err) {
    console.error(err);
  }
};

// Column definitions
const columnDefs = [
  {
    headerName: "Icon",
    field: "icon",
    cellRenderer: (params) =>
      `<img src="${params.value}" style="height: 70px; width: 50px;" alt="Movie icon" />`,
    flex: 1,
    cellStyle: { "text-align": "left" },
  },
  {
    headerName: "Name",
    field: "name",
    flex: 1,
    cellStyle: { "text-align": "left" },
    sortable: true,
  },
  {
    headerName: "Release Date",
    field: "releaseDate",
    flex: 1,
    cellStyle: { "text-align": "left" },
    sortable: true,
  },
  {
    headerName: "Description",
    field: "description",
    flex: 1,
    cellStyle: { "text-align": "left" },
    sortable: true,
  },
  {
    headerName: "Genre",
    field: "genre",
    flex: 1,
    cellStyle: { "text-align": "left" },
    sortable: true,
  },
  {
    field: "Edit",
    flex: 1,
    cellStyle: { "text-align": "left" },
    cellRenderer: "actionButtonsCellRenderer",
  },
];

const gridOptions = {
  columnDefs: columnDefs,
  components: {
    actionButtonsCellRenderer: ActionButtonsCellRenderer,
  },
  pagination: true,
  paginationPageSize: 7,
};

const onGridReady = (params) => {
  gridApi.value = params.api;
};

const onSearch = () => {
  gridApi.value.setQuickFilter(searchText.value);
};

const addMovie = () => {
  router.push({ name: "movies-add" });
};

onMounted(loadMovies);
</script>

<style scoped>
.ag-theme-alpine {
  width: 100%;
  height: 600px;
}

.green-btn {
  padding: 0.5em 1em;
  background: #41dc8e;
  color: #fff;
  border: none;
  cursor: pointer;
  border-radius: 5px;
  width: 5rem;
}

.green-btn:hover {
  background: #198450;
}

.ag-theme-alpine {
  width: 100%;
  height: 400px;
}
</style>
