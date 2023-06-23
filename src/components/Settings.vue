<style>
.page-list {
  display: flex;
  overflow-x: auto;
  padding: 10px;
  box-sizing: border-box;
  gap: 10px;
  background-color: var(--fg-h);
  border-radius: var(--brr);
}

.page-item {
  padding: 10px;
  font-weight: 600;
  text-shadow: 0 0 8px black;
  text-align: center;
  border-radius: var(--brr);
}

.page-item button {
  padding: 5px 10px;
}
</style>

<template>
  <div class="settings-bar bar">
    SETTINGS
    <button
      class="settings-top-save flex"
      style="float: right"
      :disabled="
        JSON.stringify(this.originalConfig) === JSON.stringify(this.config)
      "
      @click="saveConfig"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        style="transform: rotate(0)"
      >
        <path
          fill="currentColor"
          d="M15 9H5V5h10m-3 14a3 3 0 0 1-3-3a3 3 0 0 1 3-3a3 3 0 0 1 3 3a3 3 0 0 1-3 3m5-16H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V7l-4-4Z"
        />
      </svg>
      <span>Save</span>
    </button>
  </div>
  <section>
    <div
      class="settings-sub-bar sub-bar"
      @click="showDefHeaders = !showDefHeaders"
    >
      <div class="flex">
        <svg
          :class="showDefHeaders ? 'open' : ''"
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
        >
          <path
            fill="currentColor"
            d="M7.41 8.58L12 13.17l4.59-4.59L18 10l-6 6l-6-6l1.41-1.42Z"
          />
        </svg>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="30"
          height="24"
          viewBox="0 0 16 16"
          style="transform: rotate(0deg)"
        >
          <g fill="currentColor">
            <path
              d="M14.5 3a.5.5 0 0 1 .5.5v9a.5.5 0 0 1-.5.5h-13a.5.5 0 0 1-.5-.5v-9a.5.5 0 0 1 .5-.5h13zm-13-1A1.5 1.5 0 0 0 0 3.5v9A1.5 1.5 0 0 0 1.5 14h13a1.5 1.5 0 0 0 1.5-1.5v-9A1.5 1.5 0 0 0 14.5 2h-13z"
            />
            <path
              d="M3 8.5a.5.5 0 0 1 .5-.5h9a.5.5 0 0 1 0 1h-9a.5.5 0 0 1-.5-.5zm0 2a.5.5 0 0 1 .5-.5h6a.5.5 0 0 1 0 1h-6a.5.5 0 0 1-.5-.5zm0-5a.5.5 0 0 1 .5-.5h9a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-9a.5.5 0 0 1-.5-.5v-1z"
            />
          </g>
        </svg>
        Default Request Headers
      </div>
    </div>
    <div v-show="showDefHeaders">
      <div class="headers-input">
        <div class="header-names">
          <span>Header Names</span>
          <input
            v-model="header.name"
            placeholder="Enter header name..."
            v-for="(header, index) in config.defaults.headers"
            :key="index"
          />
        </div>
        <div class="header-values">
          <span>Header Values</span>
          <input
            v-model="header.value"
            placeholder="Enter header value..."
            v-for="(header, index) in config.defaults.headers"
            :key="index"
          />
        </div>
        <div class="header-clear">
          <span>Clear</span>
          <button
            @click="removeHeader(index)"
            v-for="(header, index) in config.defaults.headers"
            style="--_shadow: red; padding: 5px; color: red"
            :key="index"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
            >
              <path
                fill="none"
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M4 7h16m-10 4v6m4-6v6M5 7l1 12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2l1-12M9 7V4a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v3"
              />
            </svg>
          </button>
        </div>
      </div>
      <button type="button" class="add-header" @click="addHeader">+</button>
    </div>
  </section>
  <section>
    <div class="settings-sub-bar sub-bar" @click="showDefBody = !showDefBody">
      <div class="flex">
        <svg
          :class="showDefBody ? 'open' : ''"
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
        >
          <path
            fill="currentColor"
            d="M7.41 8.58L12 13.17l4.59-4.59L18 10l-6 6l-6-6l1.41-1.42Z"
          />
        </svg>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="30"
          height="24"
          viewBox="0 0 16 16"
          style="transform: rotate(0deg)"
        >
          <path
            fill="currentColor"
            d="M6.923 1.378a3 3 0 0 1 2.154 0l4.962 1.908a1.5 1.5 0 0 1 .961 1.4v6.626a1.5 1.5 0 0 1-.961 1.4l-4.962 1.909a3 3 0 0 1-2.154 0l-4.961-1.909a1.5 1.5 0 0 1-.962-1.4V4.686a1.5 1.5 0 0 1 .962-1.4l4.961-1.908Zm1.795.933a2 2 0 0 0-1.436 0l-1.384.533l5.59 2.116l1.948-.834L8.718 2.31ZM14 4.971L8.5 7.33v6.428c.074-.019.146-.042.218-.07l4.962-1.908a.5.5 0 0 0 .32-.467v-6.34Zm-6.5 8.786V7.33L2 4.972v6.34a.5.5 0 0 0 .32.467l4.962 1.908c.072.028.144.051.218.07ZM2.564 4.126L8 6.456l2.164-.928l-5.667-2.146l-1.933.744Z"
          />
        </svg>
        Default Request Body
      </div>
    </div>
    <textarea
      placeholder="Put your DEFAULT request body here..."
      class="request-body"
      v-show="showDefBody"
      v-model="config.defaults.body"
    >
    </textarea>
  </section>
  <hr />
  <section>
    <div
      class="settings-sub-bar sub-bar"
      @click="showPageOrder = !showPageOrder"
    >
      <div class="flex">
        <svg
          :class="showPageOrder ? 'open' : ''"
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
        >
          <path
            fill="currentColor"
            d="M7.41 8.58L12 13.17l4.59-4.59L18 10l-6 6l-6-6l1.41-1.42Z"
          />
        </svg>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="30"
          height="24"
          viewBox="0 0 24 24"
          style="transform: rotate(0)"
        >
          <path
            fill="currentColor"
            d="m19 2l-5 4.5v11l5-4.5V2M6.5 5C4.55 5 2.45 5.4 1 6.5v14.66c0 .25.25.5.5.5c.1 0 .15-.07.25-.07c1.35-.65 3.3-1.09 4.75-1.09c1.95 0 4.05.4 5.5 1.5c1.35-.85 3.8-1.5 5.5-1.5c1.65 0 3.35.31 4.75 1.06c.1.05.15.03.25.03c.25 0 .5-.25.5-.5V6.5c-.6-.45-1.25-.75-2-1V19c-1.1-.35-2.3-.5-3.5-.5c-1.7 0-4.15.65-5.5 1.5V6.5C10.55 5.4 8.45 5 6.5 5Z"
          />
        </svg>
        Page order
      </div>
    </div>
    <div
      class="flex"
      style="width: 100%; padding: 10px 10px 0; box-sizing: border-box"
      v-show="showPageOrder"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="30"
        height="30"
        viewBox="0 0 24 24"
        style="
          transform: rotate(0);
          margin: 5px 0px 10px 10px;
          color: var(--main-h);
        "
      >
        <path fill="currentColor" d="M8 5.14v14l11-7l-11-7Z" />
      </svg>
      <input
        placeholder="Startup Page..."
        class="url-input not-url"
        v-model="config.defaults.page"
      />
    </div>
    <div class="page-list" v-show="showPageOrder">
      <div
        v-for="(page, index) in config.pages"
        :style="`background: var(--${page.toLowerCase()})`"
        :key="index"
        class="page-item"
      >
        {{ page }}
        <div>
          <button @click="movePageUp(index)" :disabled="index === 0">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
            >
              <path
                fill="currentColor"
                d="M20 10v4h-9l3.5 3.5l-2.42 2.42L4.16 12l7.92-7.92L14.5 6.5L11 10h9Z"
              />
            </svg>
          </button>
          <button
            @click="movePageDown(index)"
            style="margin-left: 5px"
            :disabled="index === config.pages.length - 1"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
            >
              <path
                fill="currentColor"
                d="M4 10v4h9l-3.5 3.5l2.42 2.42L19.84 12l-7.92-7.92L9.5 6.5L13 10H4Z"
              />
            </svg>
          </button>
        </div>
      </div>
    </div>
  </section>
  <section>
    <div
      class="settings-sub-bar sub-bar"
      @click="showCustomCss = !showCustomCss"
    >
      <div class="flex">
        <svg
          :class="showCustomCss ? 'open' : ''"
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
        >
          <path
            fill="currentColor"
            d="M7.41 8.58L12 13.17l4.59-4.59L18 10l-6 6l-6-6l1.41-1.42Z"
          />
        </svg>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="30"
          height="24"
          viewBox="0 0 24 24"
          style="transform: rotate(0)"
        >
          <path
            fill="currentColor"
            d="m5 3l-.65 3.34h13.59L17.5 8.5H3.92l-.66 3.33h13.59l-.76 3.81l-5.48 1.81l-4.75-1.81l.33-1.64H2.85l-.79 4l7.85 3l9.05-3l1.2-6.03l.24-1.21L21.94 3H5Z"
          />
        </svg>
        Custom Css
      </div>
    </div>
    <textarea
      placeholder="Custom CSS here..."
      class="request-body"
      v-show="showCustomCss"
      v-model="config.css"
    >
    </textarea>
  </section>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
export default {
  emits: ['configUpdated'],
  data() {
    return {
      config: {
        defaults: {
          headers: [],
          body: "",
          page: "",
        },
        pages: [],
        css: "",
      },
      originalConfig: {},
      showDefBody: false,
      showDefHeaders: false,
      showPageOrder: false,
      showCustomCss: false,
    };
  },
  mounted() {
    this.getConfigValues();
  },
  methods: {
    async getConfigValues() {
      const returned = JSON.parse(await invoke("get_config_values"));
      this.config = returned;
      this.originalConfig = JSON.parse(JSON.stringify(returned));
    },
    async saveConfig() {
      console.log(
        this.config,
        this.originalConfig,
        JSON.stringify(this.originalConfig) === JSON.stringify(this.config)
      );
      await invoke("save_config", { args: JSON.stringify(this.config) });
      this.getConfigValues();
      this.$emit("configUpdated");
    },
    addHeader() {
      this.config.defaults.headers.push({ name: "", value: "" });
    },
    removeHeader(index) {
      if (index > -1) {
        this.config.defaults.headers.splice(index, 1);
      }
    },
    movePageUp(index) {
      if (index > 0) {
        [this.config.pages[index - 1], this.config.pages[index]] = [
          this.config.pages[index],
          this.config.pages[index - 1],
        ];
      }
    },
    movePageDown(index) {
      if (index < this.config.pages.length - 1) {
        [this.config.pages[index], this.config.pages[index + 1]] = [
          this.config.pages[index + 1],
          this.config.pages[index],
        ];
      }
    },
  },
};
</script>
