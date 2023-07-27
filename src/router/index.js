import { createRouter, createWebHashHistory } from "vue-router";
import LandingPage from "../pages/landing.vue";

const routes = [
  {
    path: "/",
    component: LandingPage, 
  },
  {
    path: "/http/",
    component: () => import ("../pages/HttpRequests.vue"),
  },
  {
    path: "/settings/",
    component: () => import ("../pages/SettingsPage.vue"),
  },
];

export const router = createRouter({
  history: createWebHashHistory(),
  routes,
});
