import { createApp } from "vue";
import VueHead from "vue-head";
import "./base.css";
import "./styles.css";
import "./assets/github.css";
import App from "./App.vue";
import hljs from "highlight.js/lib/core";
import json from "highlight.js/lib/languages/json";
import http from "highlight.js/lib/languages/http";
import js from "highlight.js/lib/languages/javascript";
import css from "highlight.js/lib/languages/css";
import xml from "highlight.js/lib/languages/xml";
import hljsVuePlugin from "@highlightjs/vue-plugin";

hljs.registerLanguage("json", json);
hljs.registerLanguage("http", http);
hljs.registerLanguage("xml", xml);
hljs.registerLanguage("javascript", js);
hljs.registerLanguage("css", css);

createApp(App).use(hljsVuePlugin).use(VueHead).mount("#app");
