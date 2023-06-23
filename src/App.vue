<template>
  <div class="container">
    <aside>
      <nav>
        <button
          v-for="(page, index) in config.pages"
          :key="index"
          class="page-button"
          :class="
            current_page == page ? 'page-open' : '' + ` ${page.toLowerCase()}`
          "
          :style="
            current_page == page
              ? `background-color:var(--${page.toLowerCase()}-m)`
              : ''
          "
          @click="changePage(page)"
        >
          {{ page }}
        </button>
      </nav>
      <nav>
        <div
          style="
            background-color: white;
            border-radius: 3px;
            height: 3px;
            width: 25px;
          "
        ></div>
        <button
          class="page-button"
          :class="current_page == 'LANDING' ? 'page-open' : ''"
          @click="changePage('LANDING')"
          :style="current_page == 'LANDING' ? 'color: var(--main-h)' : ''"
        >
        L
        </button>
        <button
          class="page-button"
          :class="current_page == 'SETTINGS' ? 'page-open' : ''"
          @click="changePage('SETTINGS')"
          :style="current_page == 'SETTINGS' ? 'color: var(--main-h)' : ''"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="30"
            height="40"
            viewBox="0 0 512 512"
          >
            <path
              fill="currentColor"
              d="M256 176a80 80 0 1 0 80 80a80.24 80.24 0 0 0-80-80Zm172.72 80a165.53 165.53 0 0 1-1.64 22.34l48.69 38.12a11.59 11.59 0 0 1 2.63 14.78l-46.06 79.52a11.64 11.64 0 0 1-14.14 4.93l-57.25-23a176.56 176.56 0 0 1-38.82 22.67l-8.56 60.78a11.93 11.93 0 0 1-11.51 9.86h-92.12a12 12 0 0 1-11.51-9.53l-8.56-60.78A169.3 169.3 0 0 1 151.05 393L93.8 416a11.64 11.64 0 0 1-14.14-4.92L33.6 331.57a11.59 11.59 0 0 1 2.63-14.78l48.69-38.12A174.58 174.58 0 0 1 83.28 256a165.53 165.53 0 0 1 1.64-22.34l-48.69-38.12a11.59 11.59 0 0 1-2.63-14.78l46.06-79.52a11.64 11.64 0 0 1 14.14-4.93l57.25 23a176.56 176.56 0 0 1 38.82-22.67l8.56-60.78A11.93 11.93 0 0 1 209.94 26h92.12a12 12 0 0 1 11.51 9.53l8.56 60.78A169.3 169.3 0 0 1 361 119l57.2-23a11.64 11.64 0 0 1 14.14 4.92l46.06 79.52a11.59 11.59 0 0 1-2.63 14.78l-48.69 38.12a174.58 174.58 0 0 1 1.64 22.66Z"
            />
          </svg>
        </button>
      </nav>
    </aside>
    <main>
      <div v-show="current_page === 'GET'">
        <Get :key="keys"></Get>
      </div>
      <div v-show="current_page === 'POST'">
        <Post :key="keys"></Post>
      </div>
      <div v-show="current_page === 'PATCH'">
        <Patch :key="keys"></Patch>
      </div>
      <div v-show="current_page === 'PUT'">
        <Put :key="keys"></Put>
      </div>
      <div v-show="current_page === 'DEL'">
        <Del :key="keys"></Del>
      </div>
      <div v-show="current_page === 'TRACE'">
        <Trace :key="keys"></Trace>
      </div>
      <div v-show="current_page === 'SETTINGS'">
        <Settings @configUpdated="updatedConfig"></Settings>
      </div>
      <div v-if="current_page === 'LANDING'">
        <Landing></Landing>
      </div>
      <div v-else>
      </div>
    </main>
  </div>
</template>

<script>
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import Get from "./components/Get.vue";
import Post from "./components/Post.vue";
import Patch from "./components/Patch.vue";
import Put from "./components/Put.vue";
import Del from "./components/Delete.vue";
import Trace from "./components/Trace.vue";
import Settings from "./components/Settings.vue";
import Landing from "./components/Landing.vue";

export default defineComponent({
  components: {
    Get,
    Post,
    Put,
    Patch,
    Del,
    Trace,
    Settings,
    Landing,
  },
  data() {
    return {
      current_page: "GET",
      config: {
        defaults: {
          headers: [],
          body: "",
          page: "GET",
        },
        pages: [],
        css:"",
      },
      keys:0,
    };
  },
  mounted() {
    this.updatedConfig();
  },
  methods: {
    updatedConfig() {
      this.keys++;
      this.checkConfig().then(() => {
        this.goThroughConfig();
      });
    },
    changePage(page) {
      this.current_page = page;
    },
    async checkConfig() {
      this.config = JSON.parse(await invoke("get_config_values"));
      console.log("Config:", this.config);
    },
    goThroughConfig() {
      this.changePage(this.config.defaults.page);
      this.headStyle(this.config.css)
    },
    headStyle(newCss) {
      const head = document.querySelector('head');
      const existingStyle = document.getElementById('style');

      if (existingStyle) {
        existingStyle.textContent = newCss;
      } else {
        const style = document.createElement('style');
        style.id = 'style';
        style.textContent = newCss;
        head.appendChild(style);
      }
    },
  },
});
</script>
