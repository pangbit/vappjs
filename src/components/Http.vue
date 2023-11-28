<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/primitives";

const xReqMethodURL = ref("");
const xReqHeaders = ref("");
const XReqBody = ref("");
const xResolve = ref("");
const xUseProxy = ref(false);
const xUseRedirect = ref(false);
const xSkipCertsCheck = ref(true);
const xResp = ref("");

async function send() {
    try {
        //解析method & url
        let rawMethodURL = xReqMethodURL.value.trim();
        if (rawMethodURL == "") {
            return;
        }

        let sep = rawMethodURL.indexOf(" ");
        if (sep == -1) {
            return;
        }

        let req_method = rawMethodURL.substring(0, sep).trim();
        let req_url = rawMethodURL.substring(sep).trim();

        //解析headers
        let req_headers = {};
        if (xReqHeaders.value != "") {
            let lines = xReqHeaders.value.split("\n");
            for (let i = 0; i < lines.length; i++) {
                const line = lines[i].trim();
                console.log(line);

                let kv = line.split(":");
                if (kv.length != 2) {
                    console.log("Invalid header");
                    continue;
                }

                const key = kv[0];
                const val = kv[1];

                req_headers[key] = val;
            }
        }

        //执行请求
        let args = {
            url: req_url,
            method: req_method,
            headers: req_headers,
            body: XReqBody.value,
            useProxy: xUseProxy.value,
            useRedirect: xUseRedirect.value,
            useResolve: xResolve.value,
            useCertsCheck: !xSkipCertsCheck.value
        };

        let resp = await invoke("http_request", args);
        let status = resp[0];
        let headers = resp[1];
        let body = resp[2];
        let addr = resp[3];

        let content = "DNS Resolve Addr: " + addr + "\r\n\r\n" + status.trim() + "\r\n";
        for (const key in headers) {
            if (Object.hasOwnProperty.call(headers, key)) {
                const value = headers[key];
                content = content + key + ": " + value + "\r\n"
            }
        }
        content = content + "\r\n";
        content = content + body;

        xResp.value = content;
    } catch (error) {
        xResp.value = error;
    }
}

</script>

<template>
    <form class="row" @submit.prevent="send">
        <input id="regex-input" v-model="xReqMethodURL" placeholder="GET http://www.test.com">
        <input v-model="xResolve" placeholder="DNS Resolve" style="margin-right: 5px;">
        <button type="submit">Go</button>
    </form>
    <div style="margin-bottom: 1vh;">
        <input type="checkbox" v-model="xUseProxy"><label>使用系统代理</label>
        <input type="checkbox" v-model="xUseRedirect"><label>执行重定向</label>
        <input type="checkbox" v-model="xSkipCertsCheck"><label>忽略证书校验</label>
    </div>

    <label>请求头</label>
    <textarea class="row" rows="5" v-model="xReqHeaders"></textarea>
    <label>请求体</label>
    <textarea class="row" rows="5" v-model="XReqBody"></textarea>

    <label>响应</label>
    <textarea class="row" rows="18" readonly>{{ xResp }}</textarea>
</template>
