<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

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
    <div class="box">
        <form class="form-box" @submit.prevent="send">
            <input class="form-input" v-model="xReqMethodURL" placeholder="GET https://one.one.one.one">
            <input class="form-input input-dns" v-model="xResolve" placeholder="1.1.1.1:443">
            <button class="btn form-btn" type="submit">Go</button>
        </form>
        <div class="checkboxs">
            <label for="useproxy"><input id="useproxy" type="checkbox" v-model="xUseProxy"><span>使用系统代理</span></label>
            <label for="redirect"><input id="redirect" type="checkbox" v-model="xUseRedirect"><span>执行重定向</span></label>
            <label for="skipcert"><input id="skipcert" type="checkbox"
                    v-model="xSkipCertsCheck"><span>忽略证书校验</span></label>
        </div>

        <label>请求头</label>
        <textarea class="show" rows="5" v-model="xReqHeaders" placeholder="User-Agent:vappjs/1.0.0
X-Forwarded-For:1.1.1.1
Content-Type:application/json
Content-Length:43"></textarea>
        <label>请求体</label>
        <textarea class="show" rows="5" v-model="XReqBody"
            placeholder='{"name":"Alice","age":16,"gender":"female"}'></textarea>

        <label>响应</label>
        <textarea class="show" rows="18" readonly>{{ xResp }}</textarea>
    </div>
</template>

<style scoped>
.input-dns {
    flex: 5;
}

.checkboxs {
    display: flex;
    flex-direction: row;
}

.checkboxs input {
    vertical-align: middle;
}

.checkboxs label span {
    vertical-align: middle;
    margin-right: 10px;
}
</style>