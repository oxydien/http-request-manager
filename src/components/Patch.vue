<template>
  <section>
    <div class="patch-bar bar">
      <div class="flex" @click="showForm = !showForm">
        <svg
          :class="showForm ? 'open' : ''"
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
        PATCH Request
      </div>
      <div class="flex" style="float: right">
        <input
          class="top-request-num"
          placeholder="Number of requests"
          title="Number of requests"
          max="600"
          min="1"
          type="number"
          v-model="perSecond"
        />
        <button class="top-submit" @click="send">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="18"
            height="24"
            viewBox="-2 0 24 24"
            v-if="!requestsInProgress"
          >
            <path fill="currentColor" d="m2 21l21-9L2 3v7l15 2l-15 2v7Z" />
          </svg>
          <svg
            v-else
            xmlns="http://www.w3.org/2000/svg"
            width="20"
            height="20"
            viewBox="0 0 22 22"
          >
            <path
              fill="none"
              stroke="currentColor"
              stroke-dasharray="15"
              stroke-dashoffset="15"
              stroke-linecap="round"
              stroke-width="2"
              d="M12 3C16.9706 3 21 7.02944 21 12"
            >
              <animate
                fill="freeze"
                attributeName="stroke-dashoffset"
                dur="0.3s"
                values="15;0"
              />
              <animateTransform
                attributeName="transform"
                dur="0.8s"
                repeatCount="indefinite"
                type="rotate"
                values="0 12 12;360 12 12"
              />
            </path>
          </svg>
        </button>
      </div>
    </div>

    <div class="request-form" v-show="showForm" style="padding: 20px 0">
      <div class="flex">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          style="margin: 5px 0 10px 10px; color: var(--patch)"
          width="30"
          height="30"
          viewBox="0 0 24 24"
        >
          <path
            fill="currentColor"
            d="M10.59 13.41c.41.39.41 1.03 0 1.42c-.39.39-1.03.39-1.42 0a5.003 5.003 0 0 1 0-7.07l3.54-3.54a5.003 5.003 0 0 1 7.07 0a5.003 5.003 0 0 1 0 7.07l-1.49 1.49c.01-.82-.12-1.64-.4-2.42l.47-.48a2.982 2.982 0 0 0 0-4.24a2.982 2.982 0 0 0-4.24 0l-3.53 3.53a2.982 2.982 0 0 0 0 4.24m2.82-4.24c.39-.39 1.03-.39 1.42 0a5.003 5.003 0 0 1 0 7.07l-3.54 3.54a5.003 5.003 0 0 1-7.07 0a5.003 5.003 0 0 1 0-7.07l1.49-1.49c-.01.82.12 1.64.4 2.43l-.47.47a2.982 2.982 0 0 0 0 4.24a2.982 2.982 0 0 0 4.24 0l3.53-3.53a2.982 2.982 0 0 0 0-4.24a.973.973 0 0 1 0-1.42Z"
          />
        </svg>
        <input
          id="urlInputPatch"
          autocomplete="off"
          v-model="url"
          @keyup.enter="send"
          class="url-input"
          placeholder="Enter URL..."
        />
      </div>
      <section>
        <div
          class="patch-sub-bar sub-bar"
          @click="showReqHeaders = !showReqHeaders"
        >
          <div class="flex">
            <svg
              :class="showReqHeaders ? 'open' : ''"
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
            Request Headers
          </div>
        </div>
        <div v-show="showReqHeaders">
          <div class="headers-input">
            <div class="header-names">
              <span>Header Names</span>
              <input
                v-model="header.name"
                placeholder="Enter header name..."
                v-for="(header, index) in req_headers"
                :key="index"
              />
            </div>
            <div class="header-values">
              <span>Header Values</span>
              <input
                v-model="header.value"
                placeholder="Enter header value..."
                v-for="(header, index) in req_headers"
                :key="index"
              />
            </div>
            <div class="header-clear">
              <span>Clear</span>
              <button
                @click="removeHeader(index)"
                v-for="(header, index) in req_headers"
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
        <div class="patch-sub-bar sub-bar" @click="showReqBody = !showReqBody">
          <div class="flex">
            <svg
              :class="showReqBody ? 'open' : ''"
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
            Request Body
          </div>
        </div>
        <div v-show="showReqBody">
          <textarea
            placeholder="Put your request body here..."
            class="request-body"
            v-model="body"
          >
          </textarea>
        </div>
      </section>
      <hr style="color: var(--patch-m); border-color: var(--patch-m)" />
      <button
        type="submit"
        style="--_background: var(--patch-gradient)"
        class="submit-request"
        @click="send"
        :key="responses.length"
      >
        {{ !requestsInProgress ? "Send Request" : "Cancel Request" }}
      </button>
    </div>
  </section>
  <section>
    <div class="bar patch-bar" v-show="responses.length > 0">
      <div class="flex" @click="showAllRequests = !showAllRequests">
        <svg
          :class="showAllRequests ? 'open' : ''"
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
        {{
          responses.length > 1 ? `Responses (${responses.length})` : "Reponse"
        }}
      </div>
      <span class="ping" v-show="allStartTime != 0">
        <span v-if="allEndTime != 0"> {{ allEndTime - allStartTime }}ms </span>
        <svg
          v-else
          xmlns="http://www.w3.org/2000/svg"
          width="20"
          height="20"
          viewBox="0 0 22 22"
        >
          <path
            fill="none"
            stroke="currentColor"
            stroke-dasharray="15"
            stroke-dashoffset="15"
            stroke-linecap="round"
            stroke-width="2"
            d="M12 3C16.9706 3 21 7.02944 21 12"
          >
            <animate
              fill="freeze"
              attributeName="stroke-dashoffset"
              dur="0.3s"
              values="15;0"
            />
            <animateTransform
              attributeName="transform"
              dur="0.8s"
              repeatCount="indefinite"
              type="rotate"
              values="0 12 12;360 12 12"
            />
          </path>
        </svg>
      </span>
    </div>
    <div class="all-responses" :key="responses.length">
      <section
        class="one-response"
        v-for="(oneResponse, index) in responses"
        :key="index * requests.length"
        v-show="showAllRequests"
      >
        <div
          class="mid-bar"
          :style="`--_background: var(--status-${oneResponse.statusCode}-color)`"
        >
          <div class="flex" @click="oneResponse.show = !oneResponse.show">
            <svg
              :class="oneResponse.show ? 'open' : ''"
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
            {{ index + 1 }}. Response {{ oneResponse.statusCode }} -
            {{ statusCodes[oneResponse.statusCode] }}
          </div>
          <span class="ping" v-show="oneResponse.pingTime != 0"
            >{{ oneResponse.pingTime }}ms</span
          >
        </div>
        <div class="response-main" v-show="oneResponse.show">
          <section>
            <div
              class="patch-sub-bar sub-bar sub-mid-bar"
              @click="oneResponse.showHeaders = !oneResponse.showHeaders"
            >
              <div class="flex">
                <svg
                  :class="oneResponse.showHeaders ? 'open' : ''"
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
                Response Headers
              </div>
            </div>
            <div v-show="oneResponse.showHeaders">
              <ul class="response-headers">
                <li v-for="(header, index) in oneResponse.headers" :key="index">
                  <span
                    :style="`color: var(--status-${oneResponse.statusCode}-color); filter:brightness(120%)`"
                    >{{ header[0] }}:</span
                  >
                  {{ header[1] }}
                </li>
              </ul>
            </div>
          </section>
          <section>
            <div
              class="patch-sub-bar sub-bar sub-mid-bar"
              @click="oneResponse.showBody = !oneResponse.showBody"
            >
              <div class="flex">
                <svg
                  :class="oneResponse.showBody ? 'open' : ''"
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
                Response Body
              </div>
            </div>
            <highlightjs
              autodetect
              :code="oneResponse.text || 'RM-WARN: no body'"
              class="response-body"
              v-show="oneResponse.showBody"
            ></highlightjs>
          </section>
        </div>
      </section>
    </div>
  </section>
</template>

<script setup>
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const requestsInProgress = ref(false);
const requests = [];
const url = ref("");
const body = ref("");
const req_headers = ref([]);

const config = ref({
  defaults: {
    headers: [],
    body: "",
    page: "GET",
  },
  pages: [],
  css: "",
});

const responses = ref([]);

const perSecond = ref(1);

const allStartTime = ref(0);
const allEndTime = ref(0);

const showForm = ref(true);
const showReqHeaders = ref(true);
const showReqBody = ref(false);
const showAllRequests = ref(true);

function addHeader() {
  req_headers.value.push({ name: "", value: "" });
}

const removeHeader = (index) => {
  if (index > -1) {
    req_headers.value.splice(index, 1);
  }
};

async function send() {
  if (requestsInProgress.value) {
    clearRequests();
    return;
  }
  sendRequest();
  console.log("REQUESTS:", requests);
}

async function sendRequest() {
  clearRequests();
  allStartTime.value = Date.now();
  requestsInProgress.value = true;

  const requestPromises = [];

  for (let i = 1; i <= (perSecond.value > 600 ? 600 : perSecond.value); i++) {
    requestPromises.push(doRequest(i));
  }

  try {
    await Promise.all(requestPromises);
    allEndTime.value = Date.now();
  } catch (error) {
    console.error("Error occurred while sending requests:", error);
  }

  requestsInProgress.value = false;
}

async function checkConfig() {
  config.value = JSON.parse(await invoke("get_config_values"));
  req_headers.value = config.value.defaults.headers;
  body.value = config.value.defaults.body;
}
checkConfig();

function isHttp(url) {
  if (!url.startsWith("http://") && !url.startsWith("https://")) {
    return "http://" + url;
  } else {
    return url;
  }
}

async function doRequest(index) {
  console.log("Doing request " + index + "/" + perSecond.value);
  if (url.value === "") {
    document.getElementById("urlInputPatch").style.outline = "3px solid red";
    setTimeout(() => {
      document.getElementById("urlInputPatch").style.outline = "";
    }, 3000);
    return;
  }
  url.value = isHttp(url.value);
  if (!responses.value[index - 1]) {
    responses.value[index - 1] = {}; // Create a new object if it doesn't exist
  }
  responses.value[index - 1].index = index - 1;
  try {
    const headersArray = getValidHeaders();
    const result = await makeRequest(headersArray);
    handleResponse(result, index - 1);
  } catch (error) {
    responses.value[index - 1].text = error;
  } finally {
    const endTime = Date.now();
    console.log(responses.value);
    console.log(responses.value[index - 1]);
    responses.value[index - 1].pingTime = endTime - allStartTime.value;
  }
}

function getValidHeaders() {
  return req_headers.value
    .filter((header) => header.name !== "" && header.value !== "")
    .map((header) => [header.name, header.value]);
}

async function makeRequest(headersArray) {
  const request = {
    method: "PATCH",
    url: url.value,
    headers: headersArray.length > 0 ? headersArray : null,
    data: body.value,
  };

  requests.push(request); // Store the outgoing request in the array

  return await invoke("send_request", request);
}

function handleResponse(result, index) {
  console.log("result:", result);
  let response = {};
  response.show = false;
  response.showHeaders = false;
  response.showBody = false;

  response.statusCode = result.status_code;
  response.headers = result.headers;

  if (hasContentTypeHeader(result.headers)) {
    try {
      const parsedResult = JSON.parse(result.result);
      response.text = JSON.stringify(parsedResult, null, 2);
    } catch {
      response.text = result.result;
    }
  } else {
    response.text = result.result;
  }

  responses.value[index] = response;
}

function hasContentTypeHeader(headers) {
  return headers.some(([name]) => name.toLowerCase() === "content-type");
}

function clearRequests() {
  console.log("canceling req: " + requests.length);
  requests.length = 0;
  console.log("canceling res:" + responses.value.length);
  responses.length = [];
  allStartTime.value = 0;
  allEndTime.value = 0;
}

const statusCodes = ref({
  100: "Continue",
  101: "Switching Protocols",
  102: "Processing",
  103: "Early Hints",
  200: "OK",
  201: "Created",
  202: "Accepted",
  203: "Non-Authoritative Information",
  204: "No Content",
  205: "Reset Content",
  206: "Partial Content",
  207: "Multi-Status",
  208: "Already Reported",
  226: "IM Used",
  300: "Multiple Choices",
  301: "Moved Permanently",
  302: "Found",
  303: "See Other",
  304: "Not Modified",
  305: "Use Proxy",
  307: "Temporary Redirect",
  308: "Permanent Redirect",
  400: "Bad Request",
  401: "Unauthorized",
  402: "Payment Required",
  403: "Forbidden",
  404: "Not Found",
  405: "Method Not Allowed",
  406: "Not Acceptable",
  407: "Proxy Authentication Required",
  408: "Request Timeout",
  409: "Conflict",
  410: "Gone",
  411: "Length Required",
  412: "Precondition Failed",
  413: "Payload Too Large",
  414: "URI Too Long",
  415: "Unsupported Media Type",
  416: "Range Not Satisfiable",
  417: "Expectation Failed",
  418: "I'm a Teapot",
  421: "Misdirected Request",
  422: "Unprocessable Entity",
  423: "Locked",
  424: "Failed Dependency",
  425: "Too Early",
  426: "Upgrade Required",
  428: "Precondition Required",
  429: "Too Many Requests",
  431: "Request Header Fields Too Large",
  451: "Unavailable For Legal Reasons",
  500: "Internal Server Error",
  501: "Not Implemented",
  502: "Bad Gateway",
  503: "Service Unavailable",
  504: "Gateway Timeout",
  505: "HTTP Version Not Supported",
  506: "Variant Also Negotiates",
  507: "Insufficient Storage",
  508: "Loop Detected",
  510: "Not Extended",
  511: "Network Authentication Required",
});

defineExpose({ removeHeader });
</script>
