import { createApp } from "vue";
import { createRouter, createWebHashHistory } from "vue-router"
import 'virtual:windi.css'
import App from "./App.vue";

import Home from "./pages/home.vue";

const router = createRouter({
    history: createWebHashHistory(),
    routes: [
        {
            "path": "/",
            "component": Home,
            meta: {
                title: "Home",
                description: "Browse packages from the AUR repository"
            }
        },
        {
            "path": "/search",
            "component": () => import("./pages/search.vue"),
            meta: {
                title: "Search",
                description: "Search packages from the AUR repository"
            }
        },
        {
            "path": "/package/:repo/:name",
            "component": () => import("./pages/package.vue"),
            meta: {
                title: "Package",
            }
        },
        {
            "path": "/installed",
            "component": () => import("./pages/installed.vue"),
            meta: {
                title: "Installed",
                description: "Browse installed packages"
            }
        },
        {
            "path": "/:pathMatch(.*)*",
            "component": () => import("./pages/404.vue"),
            meta: {
                title: "404",
                description: "Page not found"
            }
        }
    ],
})

createApp(App).use(router).mount("#app");
