<script setup>
import { ref } from "vue";
import { invoke } from '@tauri-apps/api/core';

const xResult = ref("");
const xError = ref("");

// const xPwdCount = ref(10);
const xPwdLength = ref(16);
const xPwdUseUpper = ref(true);
const xPwdUseLower = ref(true);
const xPwdUseNum = ref(true);
const xPwdUseSymb = ref(false);
const xPwdStrict = ref(false);

const valueList = [];
function pushValue(value) {
    if (valueList.length >= 20) {
        valueList.shift();
    }
    valueList.push(value);
    return valueList.join("\n");
}
function pushValues(values) {
    valueList.splice(0, valueList.length + values.length - 20);
    valueList.push(...values);

    return valueList.join("\n");
}

async function genUUIDv4() {
    let v4 = await invoke("gen_uuid_v4");
    xResult.value = pushValue(v4);
}

async function genUUIDv7() {
    let v7 = await invoke("gen_uuid_v7");
    xResult.value = pushValue(v7);
}

async function genPasswords() {
    let args = {
        count: 1,
        length: parseInt(xPwdLength.value),
        numbers: xPwdUseNum.value,
        loweralpha: xPwdUseLower.value,
        upperalpha: xPwdUseUpper.value,
        symbols: xPwdUseSymb.value,
        strict: xPwdStrict.value,
    };
    try {
        let passwords = await invoke("gen_passwords", args);
        xResult.value = pushValues(passwords);
    } catch (error) {
        let now = new Date();
        xError.value = now.toLocaleString() + " " + error;
    }
}

async function clean() {
    valueList.length = 0;
    xResult.value = "";
    xError.value = "";
}

</script>


<template>
    <div class="box">
        <div class="btns">
            <button class="btn" @click="genUUIDv4">UUID_V4</button>
            <button class="btn" @click="genUUIDv7">UUID_V7</button>
            <button class="btn" @click="genPasswords">Passwords</button>
            <button class="btn" @click="clean">Clean</button>
        </div>

        <div class="pwd-box">
            <p><span>密码参数：</span>长度</p>
            <input type="range" min="8" max="64" step="1" v-model="xPwdLength">
            <p>{{ xPwdLength }}</p>

            <label class="pwd-label">
                <input type="checkbox" v-model="xPwdUseUpper">
                <span>ABC</span>
            </label>

            <label class="pwd-label">
                <input type="checkbox" v-model="xPwdUseLower">
                <span>abc</span>
            </label>

            <label class="pwd-label">
                <input type="checkbox" v-model="xPwdUseNum">
                <span>123</span>
            </label>

            <label class="pwd-label">
                <input type="checkbox" v-model="xPwdUseSymb">
                <span>#$&</span>
            </label>

            <label class="pwd-label">
                <input type="checkbox" v-model="xPwdStrict">
                <span>严格模式</span>
            </label>

        </div>
        <textarea class="show" v-model="xResult" rows="20" readonly></textarea>
        <p class="error">{{ xError }}</p>
    </div>
</template>

<style scoped>
.show {
    resize: none;
}

.error {
    color: #f37171;
}

.pwd-box {
    display: flex;
    flex-direction: row;
}

.pwd-box input[type="range"] {
    vertical-align: middle;
    margin-right: 10px;
    outline: none;
}

.pwd-box p {
    vertical-align: middle;
    margin-right: 10px;
    min-width: 20px;
}

.pwd-label {
    display: flex;
    flex-direction: row;
    margin-right: 15px;
}

.pwd-label span {
    align-content: center;
}
</style>