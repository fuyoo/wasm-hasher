# wasm-md5

> computed data's hash by webAssembly
> 
> support md5,sha1,sha2-224,sha2-356,sha2-384,sha2-512,sha3-224,sha3-256,sha3-384,sha3-512,china-sm3

| type | function | params                  | return  |
|------|----------|-------------------------|---------|
| md5  | md5      | md5(blob,callback)      | promise |
| sha1 | sha1     | sha1(blob,callback)     | promise |
| sha2 | sha2_224 | sha2_224(blob,callback) | promise |
| -    | sha2_256 | sha2_256(blob,callback) | promise |
| -    | sha2_384 | sha2_384(blob,callback) | promise |
| -    | sha2_512 | sha2_512(blob,callback) | promise |
| sha3 | sha3_224 | sha3_224(blob,callback) | promise |
| -    | sha3_256 | sha3_256(blob,callback) | promise |
| -    | sha3_384 | sha3_384(blob,callback) | promise |
| -    | sha3_512 | sha3_512(blob,callback) | promise |
| sm3  | sm3      | sm3(blob,callback)      | promise |


## How to install

```sh
npm install @fuyoo/wasm-hasher
```

## How to use

**Webpack Demo**

```html
<!DOCTYPE html>
<html>

<head>
    <meta charset="UTF-8">
    <title>MD5 ONLINE</title>
</head>

<body>
计算方式：<select id="type">
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

<button id="file">选择文件</button>
<div id="fileinfo"></div>
<div id="progress"></div>
<div id="time"></div>
</body>
<script src="index.js"></script>
</html>
```

```javascript
document.querySelector("#file")
    .addEventListener("click", function () {
        let progress = document.querySelector("#progress")
        let fileinfo = document.querySelector("#fileinfo")
        let time = document.querySelector("#time")
        let i = document.createElement("input")
        let t = document.querySelector("#type")
        i.type = "file"
        i.onchange = function () {
            reset()
            let file = (this.files[0])
            fileinfo.innerHTML = `文件名：${file.name} 共：${getSize(file.size)}`
            import ("../pkg")
                .then(pkg => {
                    console.log(pkg)
                    let start = Date.now()
                    pkg[t.value](file, (ps) => {
                        progress.innerHTML = `${t.value.toUpperCase()}计算中：已完成${(ps / 1.0).toFixed(2)}%`
                    }).then(res => {
                        progress.innerHTML = `${t.value.toUpperCase()}计算已完成`
                        time.innerHTML = `${t.value.toUpperCase()}: ${res}, 用时${(Date.now() - start) / 1000}s`
                    })
                })
        }
        i.click()

        function reset() {
            progress.innerHTML = ""
            fileinfo.innerHTML = ""
            time.innerHTML = ""
        }

        function getSize(size) {
            if (size < cf(1)) return size + "B"
            if (size >= cf(1) && size < cf(2)) return (size / cf(1)).toFixed(3) + "Kb"
            if (size >= cf(2) && size < cf(3)) return (size / cf(2)).toFixed(3) + "Mb"
            if (size >= cf(3) && size < cf(4)) return (size / cf(3)).toFixed(3) + "Gb"
            if (size >= cf(4) && size < cf(5)) return (size / cf(4)).toFixed(3) + "Tb"
            return (size / cf(1)).toFixed(3) + "Kb"

        }

        function cf(n) {
            let r = 1024
            for (let i = 1; i < n; i++) {
                r *= 1024
            }
            return r
        }
    })

```

## license

[MIT LICENSE](./LICENSE)