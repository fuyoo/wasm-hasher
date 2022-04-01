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
