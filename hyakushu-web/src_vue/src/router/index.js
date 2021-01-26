import Vue from "vue";
import VueRouter from "vue-router";
import Home from "../views/Home.vue";

Vue.use(VueRouter);

const routes = [
  {
    path: "/",
    name: "home",
    component: Home,
  },
  {
    path: "/Trial",
    name: "Trial",
    component: () =>
      import(
        /* webpackChunkName: "Trial" */ "../views/Trial.vue"
      ),
  },
];

const router = new VueRouter({
  base: process.env.VUE_APP_AEC_BASE_URL,
  routes,
});

export default router;
