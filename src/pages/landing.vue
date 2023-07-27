<style lang="scss" scoped>
.create-request {
  display: flex;
  flex-flow: column;
  svg {
    width: 80px;
    height: 80px;
  }
}
.flex-grid {
  display: flex;
  flex-flow: row wrap;
  justify-content: center;
  width: 80%;
  background-color: var(--color-super-raised-bg);
  padding: var(--gap-sm);
  border-radius: var(--radius-sm);
  margin: 0 auto;
}
[class~="e"] {
  position: absolute;
}
[class~="e"] {
  bottom: 15pt;
}
[class~="e"] {
  right: 15pt;
}
[id~="e"] {
  text-decoration: none;
}
.some-text {
  display: block;
  position: relative;
  width: fit-content;
  left: 50%;
  transform: translateX(-50%);
  margin: 30px 0;
  padding: 10px;
  text-align: center;
  border: 1px solid white;
  border-radius: 15px;
  span {
    position: absolute;
    top: -10px;
    left: 30px;
    background-color: var(--color-raised-bg);
    padding: 0 5px;
    font-weight: 600;
  }
}
</style>

<template>
  <div id="landingPage">
    <Logo style="margin-top: 50px" />
    <p class="some-text">
      <span>1.1.0</span>
      I completly rewrote this app in just 24 hours, and it's surprisingly
      functional!<br />
      If you'd like to add features, contribute it
      <a href="https://github.com/oxydien/http-request-manager">on GitHub</a
      >.<br />
      Your contributions are most welcome as we continue to enhance this
      project.
    </p>
    <div class="flex-grid">
      <Button class="create-request" link="/http/">
        <ReqTypeIcon t="http" /> Create http request</Button
      >
    </div>
    <div class="e">
      &#77;&#97;&#100;&#101;&#32;&#98;&#121;&#32;
      <a id="e">&#111;&#120;&#121;&#100;&#105;&#101;&#110;</a>
    </div>
  </div>
</template>
<script>
// This page was made in rush
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Button } from "omorphia";
import ReqTypeIcon from "../components/icons/ReqTypeIcon.vue";
import Logo from "../components/icons/Logo.vue";

export default defineComponent({
  components: {
    Button,
    ReqTypeIcon,
    Logo,
  },
  data() {
    return {
      config: {},
    };
  },
  mounted() {
    this.getConfig();
    eval(
      "\x64\x6F\x63\x75\x6D\x65\x6E\x74\x2E\x67\x65\x74\x45\x6C\x65\x6D\x65\x6E\x74\x42\x79\x49\x64\x28\x22\x65\x22\x29\x2E\x68\x72\x65\x66\x20\x3D\x20\x22\x68\x74\x74\x70\x73\x3A\x2F\x2F\x67\x69\x74\x68\x75\x62\x2E\x63\x6F\x6D\x2F\x6F\x78\x79\x64\x69\x65\x6E\x22\x3B\x64\x6F\x63\x75\x6D\x65\x6E\x74\x2E\x67\x65\x74\x45\x6C\x65\x6D\x65\x6E\x74\x42\x79\x49\x64\x28\x22\x65\x22\x29\x2E\x74\x61\x72\x67\x65\x74\x20\x3D\x20\x22\x5F\x62\x6C\x61\x6E\x6B\x22\x3B"
    );
  },
  methods: {
    async getConfig() {
      this.config = await invoke("get_config_values");
    },
  },
});
</script>
