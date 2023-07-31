<template>
  <div>
    <div>
     {{lng.lng}} <select :value="cLang" @change="setLngFn">
        <option value="zh">简体中文</option>
        <option value="en">English</option>
      </select>
    </div>
    {{ lng.at }}：<select id="type" @change="setValue">
    <optgroup label="md5">
      <option value="md5">md5</option>
    </optgroup>
    <optgroup label="sha1">
      <option value="sha1">sha1</option>
    </optgroup>
    <optgroup label="sha2">
      <option value="sha2_224">sha2-224</option>
      <option value="sha2_256">sha2-256</option>
      <option value="sha2_384">sha2-384</option>
      <option value="sha2_512">sha2-512</option>
    </optgroup>
    <optgroup label="sha3">
      <option value="sha3_224">sha3-224</option>
      <option value="sha3_256">sha3-256</option>
      <option value="sha3_384">sha3-384</option>
      <option value="sha3_512">sha3-512</option>
      <option value="sha3_512">sha3-512</option>
    </optgroup>
    <optgroup :label="lng.gm">
      <option value="sm3">sm3</option>
    </optgroup>
  </select>
    <button id="file" @click="chooseFileFn">{{lng.cf}}</button>
    <div id="hash">
      <div>{{ lng.jd }}：<input type="range" :value="ps"> {{ps}}%</div>
      <div>{{t}}{{lng.val}}：{{h}}</div>
      <div>{{lng.time}}: {{used}} ms</div>
    </div>
  </div>

</template>

<script lang="ts" setup>
import * as hash from "@fuyoo/wasm-hasher"
import {ref} from "vue";
const cLang = ref("")
const lang =  {
  zh: {
    lng: "语言",
    at:"计算方式",
    gm: "国密",
    cf:"选择文件",
    jd: "进度",
    val: "值",
    time: "耗时"
  },
  en: {
    lng: "language",
    at:"Hash Type",
    gm: "Chinese SM3",
    cf:"Choose File",
    jd: "Progress",
    val: "value",
    time: "used"
  }
}
const lng = ref<Record<string, string>>({})
if (navigator.language  === "zh-CN") {
  lng.value = lang.zh
  cLang.value = "zh"
} else {
  lng.value = lang.en
  cLang.value = "en"
}
if (localStorage.getItem("cLang")) {

  lng.value = lang[localStorage.getItem("cLang")]
  cLang.value = localStorage.getItem("cLang")
}
const setLngFn = (evt:any) => {
  lng.value = lang[evt.target.value]
  cLang.value = evt.target.value
  localStorage.setItem("cLang", cLang.value)
}
let t = ref("md5")
let ps = ref(0)
let h = ref("")
let used = ref(0)
const setValue = (evt: any) => {
  t.value = evt.target.value
}
const chooseFileFn = () => {
  const input = document.createElement("input")
  input.type = 'file'
  input.onchange = function (ev: any) {
    const file = ev.target.files[0]
    hash.default().then(() => {
      ps.value = 0
      const start = Date.now()
      //@ts-ignore
      hash[t.value](file, (p: number) => {
        ps.value = p
      })
          .then((res: string) => {
            ps.value = 100
            h.value = res
            used.value = Date.now() - start
          })
    })
  }
  input.click()
}


</script>

<style lang="scss">
</style>
