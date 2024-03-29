# wasm-hasher

> computed data's hash by webAssembly
> 
> support md5,sha1,sha2-224,sha2-356,sha2-384,sha2-512,sha3-224,sha3-256,sha3-384,sha3-512,china-sm3

| type | function | params                  | return  |
|------|----------|-------------------------|---------|
| md5  | md5      | md5(file,callback)      | promise |
| sha1 | sha1     | sha1(file,callback)     | promise |
| sha2 | sha2_224 | sha2_224(file,callback) | promise |
| -    | sha2_256 | sha2_256(file,callback) | promise |
| -    | sha2_384 | sha2_384(file,callback) | promise |
| -    | sha2_512 | sha2_512(file,callback) | promise |
| sha3 | sha3_224 | sha3_224(file,callback) | promise |
| -    | sha3_256 | sha3_256(file,callback) | promise |
| -    | sha3_384 | sha3_384(file,callback) | promise |
| -    | sha3_512 | sha3_512(file,callback) | promise |
| sm3  | sm3      | sm3(file,callback)      | promise |


## How to install

```sh
npm install @fuyoo/wasm-hasher
```

## How to use

*tips: vite should be set vite.config.js optimizeDeps:{ exclude: [
"@fuyoo/wasm-hasher"
]},like this:*

```ts
export default defineConfig({
    plugins: [vue()],
    optimizeDeps: {
        exclude: [
            "@fuyoo/wasm-hasher"
        ]
    }
})
```

```vue
<template>
  <div>
    计算方式：<select id="type" @change="setValue">
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
    <optgroup label="国密">
      <option value="sm3">sm3</option>
    </optgroup>
  </select>
    <button id="file" @click="chooseFileFn">select file</button>
    <div id="hash">
      <div>progress：<input type="range" :value="ps"> {{ps}}%</div>
      <div>{{t}}值：{{h}}</div>
    </div>
  </div>

</template>

<script lang="ts" setup>
import * as hash from "@fuyoo/wasm-hasher"
import {ref} from "vue";

let t = ref("md5")
let ps = ref(0)
let h = ref("")
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
      //@ts-ignore
      hash[t.value](file, (p: number) => {
        ps.value = p
      })
          .then((res: string) => {
            ps.value = 100
            h.value = res
          })
    })
  }
  input.click()
}
</script>

<style lang="scss">
</style>

```
## license

[MIT LICENSE](./LICENSE)
