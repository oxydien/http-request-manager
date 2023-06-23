<style scoped>
  h1 {
    text-align: center;
    text-transform: uppercase;
    line-height: 1.1;
    font-size: clamp(1rem, 5vw, 4.5rem);
  }
</style>

<template>
  <h1>
    Request <br>
    Manager
  </h1>
  <div class="pages">
    <div class="page" v-for="(page, index) in config.pages" :key="index">
      {{ page }}
    </div>
  </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
import { defineComponent } from 'vue'
export default defineComponent({
  data() {
    return {
      config: {
        pages: []
      }
    }
  },
  mounted() {
    this.checkConfig();
  },  
  methods: {
    async checkConfig() {
      this.config = JSON.parse(await invoke("get_config_values"));
    },
  }
})
</script>