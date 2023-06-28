import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import HomeView from "../views/HomeView.vue";
import MoviesView from "../views/MoviesView.vue";
import MoviesEditView from "../views/MoviesEditView.vue";
import MoviesDeleteView from "../views/MoviesDeleteView.vue";
import MoviesAddView from "../views/MoviesAddView.vue";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "home",
    component: HomeView,
  },
  {
    path: "/movies",
    name: "movies",
    component: MoviesView,
  },
  {
    path: "/movies/edit/:id",
    name: "movies-edit",
    component: MoviesEditView,
  },
  {
    path: "/movies/delete/:id",
    name: "movies-delete",
    component: MoviesDeleteView,
  },
  {
    path: "/movies/add",
    name: "movies-add",
    component: MoviesAddView,
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;
