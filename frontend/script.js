var tilesets = {};
var selectedTile = {};

function copyTile(originCanvas, tileX, tileY, destinationImage) {
    let originCtx = originCanvas.getContext("2d");
    let destinationCanvas = document.createElement("canvas");
    destinationCanvas.width = 16;
    destinationCanvas.height = 16;
    let destinationCtx = destinationCanvas.getContext("2d");
    let px = tileX * 16;
    let py = tileY * 16;
    for (let y = 0; y < 16; y++) {
        for (let x = 0; x < 16; x++) {
            let pixelData = originCtx.getImageData(px + x, py + y, 1, 1).data;
            destinationCtx.fillStyle = "rgba(" + pixelData[0] + "," + pixelData[1] + "," + pixelData[2] + "," + (pixelData[3] / 255) + ")";
            destinationCtx.fillRect(x, y, 1, 1);
        }
    }
    destinationImage.src = destinationCanvas.toDataURL();
}

window.onload = function () {
    for (let tileName of ["bottomTile", "topTile", "combinedTile"]) {
        let canvas = document.createElement("canvas");
        canvas.width = 16;
        canvas.height = 16;
        let tile = canvas.getContext("2d");
        tile.beginPath();
        tile.rect(0, 0, 16, 16);
        tile.fillStyle = "white";
        tile.fill();
        document.getElementById(tileName).src = canvas.toDataURL();
    }

    //Check File API support
    if (window.File && window.FileList && window.FileReader) {
        var filesInput = document.getElementById("files");

        filesInput.addEventListener("change", function (event) {
            var files = event.target.files; //FileList object
            var output = document.getElementById("uploads");
            output.innerHTML = "";

            for (var i = 0; i < files.length; i++) {
                var file = files[i];

                //Only pics
                if (!file.type.match('image'))
                    continue;
                const fileName = file.name;

                var picReader = new FileReader();
                picReader.addEventListener("load", function (event) {
                    var picFile = event.target;
                    var div = document.createElement("div");
                    div.setAttribute("class", "tilesetContainer");
                    const id = fileName.replace(/[\W_]+/g, "");

                    // let span = document.createElement("span");
                    // span.innerText = fileName;
                    // let canvas = document.createElement("canvas");
                    // let ctx = canvas.getContext("2d");
                    // let image = new Image();
                    // image.src = picFile.result;
                    // image.setAttribute("class", "tileset");
                    // image.setAttribute("id", id);
                    //
                    // image.onload = function () {
                    //     ctx.drawImage(image, 0, 0);
                    // };
                    //
                    // div.appendChild(span);
                    // div.appendChild(canvas);


                    div.innerHTML = "<span>" + fileName + "</span><img id='" + id + "' class='tileset' src='" + picFile.result + "'" +
                        "title='" + picFile.name + "' alt='failed to preview image'/>";

                    tilesets[id] = {fileName: fileName, div: div, data: picFile.result};
                    output.insertBefore(div, null);
                    document.getElementById(id).addEventListener('click', function (event) {
                        let bounds = this.getBoundingClientRect();
                        let left = bounds.left;
                        let top = bounds.top;
                        let x = event.pageX - left;
                        let y = event.pageY - top;
                        let cw = this.clientWidth;
                        let ch = this.clientHeight;
                        let iw = this.naturalWidth;
                        let ih = this.naturalHeight;
                        let px = x / cw * iw;
                        let py = y / ch * ih;
                        let tileX = Math.floor(px / 16);
                        let tileY = Math.floor(py / 16);
                        let tileId = Math.floor(this.naturalWidth / 16) * tileY + tileX;
                        console.log("clicked (" + tileX + ", " + tileY + ") #" + tileId);
                        let canvas = document.createElement('canvas');
                        canvas.width = this.naturalWidth;
                        canvas.height = this.naturalHeight;
                        canvas.getContext('2d').drawImage(this, 0, 0, this.naturalWidth, this.naturalHeight);
                        copyTile(canvas, tileX, tileY, document.getElementById("bottomTile"));
                        selectedTile = {imageId: id, x: tileX, y: tileY, tileId: tileId};
                    });
                });

                //Read the image
                picReader.readAsDataURL(file);
            }

        });
    } else {
        document.getElementById("uploads").innerHTML = "ERROR: Your browser does not support File API!";
    }
};